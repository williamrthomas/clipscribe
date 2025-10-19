import React from 'react';
import { Clip } from '../types';
import { Clock, CheckCircle2, Circle } from 'lucide-react';

interface ClipReviewListProps {
  clips: Clip[];
  onToggleClip: (clipId: string) => void;
  onGenerate: () => void;
}

export function ClipReviewList({ clips, onToggleClip, onGenerate }: ClipReviewListProps) {
  const selectedCount = clips.filter(c => c.isSelected).length;

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold">
          Review Clips ({selectedCount} selected)
        </h2>
        <button
          onClick={onGenerate}
          disabled={selectedCount === 0}
          className="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium"
        >
          Generate {selectedCount} Clip{selectedCount !== 1 ? 's' : ''}
        </button>
      </div>

      <div className="space-y-3">
        {clips.map((clip) => (
          <div
            key={clip.id}
            onClick={() => onToggleClip(clip.id)}
            className={`
              p-4 rounded-lg border-2 cursor-pointer transition-all
              ${clip.isSelected 
                ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20' 
                : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600'
              }
            `}
          >
            <div className="flex items-start gap-3">
              {clip.isSelected ? (
                <CheckCircle2 className="w-6 h-6 text-blue-600 dark:text-blue-400 flex-shrink-0 mt-0.5" />
              ) : (
                <Circle className="w-6 h-6 text-gray-400 flex-shrink-0 mt-0.5" />
              )}
              
              <div className="flex-1 min-w-0">
                <h3 className="font-semibold text-lg mb-2">
                  {clip.title}
                </h3>
                
                <div className="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400">
                  <Clock className="w-4 h-4" />
                  <span>
                    {clip.startTime} → {clip.endTime}
                  </span>
                </div>

                {clip.sanitizedFilename && (
                  <div className="mt-2 text-xs text-gray-500 dark:text-gray-500 font-mono">
                    {clip.sanitizedFilename}.mp4
                  </div>
                )}
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
