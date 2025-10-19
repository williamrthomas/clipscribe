import React from 'react';
import { Loader2 } from 'lucide-react';

interface ProgressIndicatorProps {
  progress?: number;
  message?: string;
}

export function ProgressIndicator({ progress, message }: ProgressIndicatorProps) {
  return (
    <div className="flex flex-col items-center justify-center py-12 space-y-6">
      <Loader2 className="w-16 h-16 text-blue-600 animate-spin" />
      
      {message && (
        <p className="text-lg text-gray-700 dark:text-gray-300">
          {message}
        </p>
      )}

      {progress !== undefined && (
        <div className="w-full max-w-md">
          <div className="flex justify-between text-sm text-gray-600 dark:text-gray-400 mb-2">
            <span>Processing clips...</span>
            <span>{Math.round(progress)}%</span>
          </div>
          <div className="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
            <div
              className="h-full bg-blue-600 transition-all duration-300 ease-out"
              style={{ width: `${progress}%` }}
            />
          </div>
        </div>
      )}
    </div>
  );
}
