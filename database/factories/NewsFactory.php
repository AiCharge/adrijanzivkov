<?php

namespace Database\Factories;

use App\Models\News;
use Illuminate\Database\Eloquent\Factories\Factory;

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\News>
 */
class NewsFactory extends Factory
{
    protected $model = News::class;

    /**
     * Define the model's default state.
     *
     * @return array<string, mixed>
     */
    public function definition(): array
    {
        return [
            'title' => fake()->sentence(6),
            'short_description' => fake()->paragraph(2),
            'main_image' => 'news/'.fake()->unique()->word().'.jpg',
            'content' => '<p>'.implode('</p><p>', fake()->paragraphs(5)).'</p>',
        ];
    }
}
