<?php

namespace App\Enums;

enum StorySubmissionStatus: string
{
    case New = 'new';
    case Accepted = 'accepted';
    case Rejected = 'rejected';
    case Completed = 'completed';

    public function label(): string
    {
        return match ($this) {
            self::New => 'Ново',
            self::Accepted => 'Прифатено',
            self::Rejected => 'Одбиено',
            self::Completed => 'Завршено',
        };
    }
}
