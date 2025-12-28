<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Facades\Storage;

class News extends Model
{
    use HasFactory;

    protected $appends = [
        'main_image_url',
    ];

    protected function mainImageUrl(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->main_image
                ? asset(Storage::url($this->main_image))
                : null,
        );
    }
}
