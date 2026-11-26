<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Http\Requests\Admin\StoreRichContentAttachmentRequest;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Storage;

class RichContentAttachmentController extends Controller
{
    public function store(StoreRichContentAttachmentRequest $request): JsonResponse
    {
        $path = $request->file('file')->store('rich-content');

        return response()->json([
            'url' => Storage::url($path),
        ], 201);
    }
}
