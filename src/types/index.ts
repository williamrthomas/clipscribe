// Clip representation
export interface Clip {
  id: string;                    // UUID
  title: string;                 // AI-generated title
  startTime: string;             // HH:MM:SS format
  endTime: string;               // HH:MM:SS format
  isSelected: boolean;           // User can toggle
  sanitizedFilename?: string;    // Safe filename version
}

// Application state
export type AppState = 
  | { status: 'ready' }
  | { status: 'analyzing' }
  | { status: 'review', clips: Clip[] }
  | { status: 'processing', progress: number }
  | { status: 'complete', outputPath: string, clipCount: number }
  | { status: 'error', message: string };

// File inputs
export interface ProjectFiles {
  videoFile: File | null;
  transcriptFile: File | null;
  context?: string;
}

// Processing result from backend
export interface ProcessingResult {
  output_directory: string;
  clip_count: number;
}

// Progress event payload
export interface ClipProgress {
  current: number;
  total: number;
}
