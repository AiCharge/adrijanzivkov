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

    public function color(): string
    {
        return match ($this) {
            self::New => 'info',
            self::Accepted => 'success',
            self::Rejected => 'danger',
            self::Completed => 'neutral',
        };
    }

    /**
     * @return list<array{value: string, label: string, color: string}>
     */
    public static function options(): array
    {
        return array_map(fn (self $status): array => [
            'value' => $status->value,
            'label' => $status->label(),
            'color' => $status->color(),
        ], self::cases());
    }
}
