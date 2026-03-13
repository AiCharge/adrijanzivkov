<?php

namespace App\Http\Controllers;

use Inertia\Inertia;
use Inertia\Response;

class LegalController extends Controller
{
    public function terms(): Response
    {
        return Inertia::render('TermsOfUse');
    }

    public function privacy(): Response
    {
        return Inertia::render('PrivacyPolicy');
    }
}
