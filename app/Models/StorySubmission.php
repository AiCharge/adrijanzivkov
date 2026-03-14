<?php

namespace App\Models;

use App\Enums\StorySubmissionStatus;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class StorySubmission extends Model
{
    use HasFactory;

    protected $fillable = [
        'name',
        'phone',
        'message',
        'status',
        'notes',
    ];

    protected $attributes = [
        'status' => StorySubmissionStatus::New->value,
    ];

    protected function casts(): array
    {
        return [
            'status' => StorySubmissionStatus::class,
        ];
    }
}
