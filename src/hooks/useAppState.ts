import { useState, useCallback, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import type { AppState, Clip, ProcessingResult, ClipProgress } from '../types';

export function useAppState() {
  const [state, setState] = useState<AppState>({ status: 'ready' });
  const [videoPath, setVideoPath] = useState<string | null>(null);
  const [transcriptPath, setTranscriptPath] = useState<string | null>(null);
  const [context, setContext] = useState<string>('');

  // Listen for progress events
  useEffect(() => {
    const setupListener = async () => {
      const unlisten = await listen<ClipProgress>('clip-progress', (event) => {
        const { current, total } = event.payload;
        setState({ status: 'processing', progress: (current / total) * 100 });
      });

      return unlisten;
    };

    const unlistenPromise = setupListener();

    return () => {
      unlistenPromise.then(fn => fn());
    };
  }, []);

  const analyzeClips = useCallback(async () => {
    if (!videoPath || !transcriptPath) return;

    setState({ status: 'analyzing' });

    try {
      const clips: Clip[] = await invoke('analyze_transcript_for_clips', {
        transcriptPath,
        videoPath,
        userContext: context || null,
      });

      // Mark all clips as selected by default
      const clipsWithSelection = clips.map(clip => ({
        ...clip,
        isSelected: true,
      }));

      setState({ status: 'review', clips: clipsWithSelection });
    } catch (error) {
      setState({ status: 'error', message: String(error) });
    }
  }, [videoPath, transcriptPath, context]);

  const generateClips = useCallback(async (selectedClips: Clip[]) => {
    if (!videoPath) return;

    setState({ status: 'processing', progress: 0 });

    try {
      const result: ProcessingResult = await invoke('generate_clips', {
        videoPath,
        clips: selectedClips,
      });

      setState({
        status: 'complete',
        outputPath: result.output_directory,
        clipCount: result.clip_count,
      });
    } catch (error) {
      setState({ status: 'error', message: String(error) });
    }
  }, [videoPath]);

  const reset = useCallback(() => {
    setState({ status: 'ready' });
    setVideoPath(null);
    setTranscriptPath(null);
    setContext('');
  }, []);

  return {
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
  };
}
