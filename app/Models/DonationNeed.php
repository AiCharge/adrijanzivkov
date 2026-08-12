<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Facades\Storage;

class DonationNeed extends Model
{
    use HasFactory;

    protected $fillable = [
        'title',
        'description',
        'sticky_content',
        'image',
        'needed_amount',
        'archived_at',
    ];

    protected function casts(): array
    {
        return [
            'archived_at' => 'datetime',
        ];
    }

    public function scopeNotArchived($query): void
    {
        $query->whereNull('archived_at');
    }

    protected $appends = [
        'image_url',
    ];

    protected function imageUrl(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->image
                ? asset(Storage::disk(config('filesystems.media.disk'))->url($this->image))
                : null,
        );
    }
}
