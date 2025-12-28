<?php

namespace Database\Seeders;

use App\Models\DonationNeed;
use Illuminate\Database\Seeder;

class DonationNeedSeeder extends Seeder
{
    /**
     * Run the database seeds.
     */
    public function run(): void
    {
        DonationNeed::factory()->count(10)->create();
    }
}
