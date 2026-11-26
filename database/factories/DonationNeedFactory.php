<?php

namespace Database\Factories;

use App\Models\DonationNeed;
use Illuminate\Database\Eloquent\Factories\Factory;

/**
 * @extends Factory<DonationNeed>
 */
class DonationNeedFactory extends Factory
{
    /**
     * Define the model's default state.
     *
     * @return array<string, mixed>
     */
    public function definition(): array
    {
        return [
            'title' => fake()->sentence(4),
            'description' => fake()->paragraphs(3, true),
            'image' => fake()->imageUrl(800, 600, 'people', true),
            'needed_amount' => fake()->numberBetween(1000, 100000),
        ];
    }
}
