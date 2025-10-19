import React, { useState } from 'react';
import { open } from '@tauri-apps/api/dialog';
import { Upload } from 'lucide-react';

interface FileDropZoneProps {
  accept: string[];  // e.g., ["mp4", "mov", "mkv"] or ["vtt", "txt"]
  label: string;
  onFileSelected: (path: string) => void;
  currentPath: string | null;
  disabled?: boolean;
}

export function FileDropZone({ accept, label, onFileSelected, currentPath, disabled }: FileDropZoneProps) {
  const [isDragging, setIsDragging] = useState(false);

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
    
    // Note: In Tauri, drag-and-drop doesn't expose file paths for security reasons
    // We need to use the file dialog instead
    alert('Please use the "Browse" button to select files.');
  };

  const handleBrowse = async () => {
    const selected = await open({
      multiple: false,
      filters: [{
        name: label,
        extensions: accept
      }]
    });
    
    if (selected && typeof selected === 'string') {
      onFileSelected(selected);
    }
  };

  return (
    <div
      onDragOver={(e) => { e.preventDefault(); setIsDragging(true); }}
      onDragLeave={() => setIsDragging(false)}
      onDrop={handleDrop}
      className={`
        relative border-2 border-dashed rounded-lg p-8 text-center transition-all
        ${isDragging ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'}
        ${disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer hover:border-blue-400'}
      `}
      onClick={disabled ? undefined : handleBrowse}
    >
      {currentPath ? (
        <div className="space-y-2">
          <div className="text-green-600 dark:text-green-400 font-medium">
            ✓ File selected
          </div>
          <div className="text-sm text-gray-600 dark:text-gray-400 truncate max-w-md mx-auto">
            {currentPath.split('/').pop()}
          </div>
        </div>
      ) : (
        <div className="space-y-3">
          <Upload className="w-12 h-12 mx-auto text-gray-400" />
          <div>
            <p className="text-lg font-medium text-gray-700 dark:text-gray-300">{label}</p>
            <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
              {accept.map(ext => `.${ext}`).join(', ')}
            </p>
          </div>
          <button 
            type="button"
            className="mt-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
            onClick={(e) => {
              e.stopPropagation();
              handleBrowse();
            }}
          >
            Browse Files
          </button>
        </div>
      )}
    </div>
  );
}
