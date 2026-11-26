export type StorySubmissionStatusColor = 'info' | 'success' | 'danger' | 'neutral';

export interface StorySubmissionStatusOptionInterface {
    value: string;
    label: string;
    color: StorySubmissionStatusColor;
}

export interface StorySubmissionInterface {
    id: number;
    name: string;
    phone: string;
    message: string;
    status: string;
    notes: string | null;
    created_at: string;
}
