import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Settings, Sparkles, FolderOpen, AlertCircle, CheckCircle2, Loader2 } from 'lucide-react';
import { useAppState } from './hooks/useAppState';
import { FileDropZone } from './components/FileDropZone';
import { SettingsModal } from './components/SettingsModal';
import { ClipReviewList } from './components/ClipReviewList';
import { ProgressIndicator } from './components/ProgressIndicator';
import type { Clip } from './types';

function App() {
  const {
    state,
    videoPath,
    setVideoPath,
    transcriptPath,
    setTranscriptPath,
    context,
    setContext,
    analyzeClips,
    generateClips,
    reset,
  } = useAppState();

  const [settingsOpen, setSettingsOpen] = useState(false);
  const [clips, setClips] = useState<Clip[]>([]);
  const [isGeneratingTranscript, setIsGeneratingTranscript] = useState(false);
  const [transcriptProgress, setTranscriptProgress] = useState('');

  // Update local clips when state changes
  React.useEffect(() => {
    if (state.status === 'review') {
      setClips(state.clips);
    }
  }, [state]);

  // Listen for transcription progress
  React.useEffect(() => {
    const setupListener = async () => {
      const { listen } = await import('@tauri-apps/api/event');
      const unlisten = await listen<string>('transcription-progress', (event) => {
        setTranscriptProgress(event.payload);
      });
      return unlisten;
    };

    const unlistenPromise = setupListener();
    return () => {
      unlistenPromise.then(fn => fn());
    };
  }, []);

  const handleToggleClip = (clipId: string) => {
    setClips(prev => prev.map(clip => 
      clip.id === clipId 
        ? { ...clip, isSelected: !clip.isSelected }
        : clip
    ));
  };

  const handleGenerate = () => {
    const selectedClips = clips.filter(c => c.isSelected);
    generateClips(selectedClips);
  };

  const handleOpenFolder = async () => {
    if (state.status === 'complete') {
      try {
        await invoke('open_in_file_explorer', { path: state.outputPath });
      } catch (error) {
        console.error('Failed to open folder:', error);
      }
    }
  };

  const handleGenerateTranscript = async () => {
    if (!videoPath) return;

    setIsGeneratingTranscript(true);
    setTranscriptProgress('Starting transcription...');

    try {
      const vttPath = await invoke<string>('generate_transcript_from_video', {
        videoPath,
      });
      setTranscriptPath(vttPath);
      setTranscriptProgress('Transcript generated!');
      setTimeout(() => {
        setIsGeneratingTranscript(false);
        setTranscriptProgress('');
      }, 1000);
    } catch (error) {
      alert(`Failed to generate transcript: ${error}`);
      setIsGeneratingTranscript(false);
      setTranscriptProgress('');
    }
  };

  const canAnalyze = videoPath && transcriptPath && state.status === 'ready';
  const canGenerateTranscript = videoPath && !transcriptPath && !isGeneratingTranscript;

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100">
      {/* Header */}
      <header className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Sparkles className="w-8 h-8 text-blue-600" />
            <h1 className="text-2xl font-bold">ClipScribe</h1>
          </div>
          <button
            onClick={() => setSettingsOpen(true)}
            className="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
            title="Settings"
          >
            <Settings className="w-6 h-6" />
          </button>
        </div>
      </header>

      {/* Main Content */}
      <main className="container mx-auto px-6 py-8 max-w-4xl">
        {state.status === 'ready' && (
          <div className="space-y-6">
            <div className="text-center mb-8">
              <h2 className="text-3xl font-bold mb-2">
                Extract Key Moments from Your Videos
              </h2>
              <p className="text-gray-600 dark:text-gray-400">
                Upload your video and transcript, let AI find the important clips
              </p>
            </div>

            <div className="grid md:grid-cols-2 gap-6">
              <FileDropZone
                accept={['mp4', 'mov', 'mkv']}
                label="Video File"
                onFileSelected={setVideoPath}
                currentPath={videoPath}
              />
              
              <FileDropZone
                accept={['vtt', 'txt']}
                label="Transcript File"
                onFileSelected={setTranscriptPath}
                currentPath={transcriptPath}
              />
            </div>

            <div>
              <label className="block text-sm font-medium mb-2">
                Context (Optional)
              </label>
              <textarea
                value={context}
                onChange={(e) => setContext(e.target.value)}
                placeholder="Add any context or guidance for clip selection (e.g., 'Focus on technical decisions' or 'Find action items')"
                className="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
                rows={3}
              />
            </div>

            {!transcriptPath && videoPath && (
              <div className="p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg">
                <p className="text-sm text-blue-700 dark:text-blue-300 mb-3">
                  💡 No transcript? We can generate one automatically using AI!
                </p>
                <button
                  onClick={handleGenerateTranscript}
                  disabled={!canGenerateTranscript || isGeneratingTranscript}
                  className="w-full py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium flex items-center justify-center gap-2"
                >
                  {isGeneratingTranscript ? (
                    <>
                      <Loader2 className="w-5 h-5 animate-spin" />
                      {transcriptProgress || 'Generating transcript...'}
                    </>
                  ) : (
                    <>
                      <Sparkles className="w-5 h-5" />
                      Generate Transcript (Whisper AI)
                    </>
                  )}
                </button>
                <p className="text-xs text-gray-500 dark:text-gray-400 mt-2 text-center">
                  Cost: ~$0.006 per minute of audio
                </p>
              </div>
            )}

            <button
              onClick={analyzeClips}
              disabled={!canAnalyze}
              className="w-full py-4 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-semibold text-lg flex items-center justify-center gap-2"
            >
              <Sparkles className="w-5 h-5" />
              Analyze & Find Clips
            </button>
          </div>
        )}

        {state.status === 'analyzing' && (
          <ProgressIndicator message="Analyzing transcript with AI..." />
        )}

        {state.status === 'review' && (
          <ClipReviewList
            clips={clips}
            onToggleClip={handleToggleClip}
            onGenerate={handleGenerate}
          />
        )}

        {state.status === 'processing' && (
          <ProgressIndicator 
            progress={state.progress}
            message="Generating video clips..."
          />
        )}

        {state.status === 'complete' && (
          <div className="text-center py-12 space-y-6">
            <CheckCircle2 className="w-20 h-20 text-green-600 mx-auto" />
            <div>
              <h2 className="text-3xl font-bold mb-2">
                Success!
              </h2>
              <p className="text-gray-600 dark:text-gray-400 text-lg">
                Generated {state.clipCount} clip{state.clipCount !== 1 ? 's' : ''}
              </p>
            </div>
            
            <div className="flex justify-center gap-4">
              <button
                onClick={handleOpenFolder}
                className="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors flex items-center gap-2 font-medium"
              >
                <FolderOpen className="w-5 h-5" />
                Show in Folder
              </button>
              <button
                onClick={reset}
                className="px-6 py-3 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-200 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors font-medium"
              >
                Process Another Video
              </button>
            </div>
          </div>
        )}

        {state.status === 'error' && (
          <div className="text-center py-12 space-y-6">
            <AlertCircle className="w-20 h-20 text-red-600 mx-auto" />
            <div>
              <h2 className="text-3xl font-bold mb-2">
                Error
              </h2>
              <p className="text-red-600 dark:text-red-400 text-lg max-w-2xl mx-auto">
                {state.message}
              </p>
            </div>
            
            <button
              onClick={reset}
              className="px-6 py-3 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-200 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors font-medium"
            >
              Try Again
            </button>
          </div>
        )}
      </main>

      <SettingsModal isOpen={settingsOpen} onClose={() => setSettingsOpen(false)} />
    </div>
  );
}

export default App;
