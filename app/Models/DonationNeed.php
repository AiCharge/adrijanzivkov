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
    ];

    protected $appends = [
        'image_url',
    ];

    protected function imageUrl(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->image
                ? asset(Storage::url($this->image))
                : null,
        );
    }
}
