<?php

namespace App\Enums;

use Filament\Support\Contracts\HasColor;
use Filament\Support\Contracts\HasLabel;

enum StorySubmissionStatus: string implements HasColor, HasLabel
{
    case New = 'new';
    case Accepted = 'accepted';
    case Rejected = 'rejected';
    case Completed = 'completed';

    public function getLabel(): string
    {
        return match ($this) {
            self::New => 'Ново',
            self::Accepted => 'Прифатено',
            self::Rejected => 'Одбиено',
            self::Completed => 'Завршено',
        };
    }

    public function getColor(): string
    {
        return match ($this) {
            self::New => 'info',
            self::Accepted => 'success',
            self::Rejected => 'danger',
            self::Completed => 'gray',
        };
    }
}
