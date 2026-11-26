<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Http\Requests\Admin\StoreNewsRequest;
use App\Http\Requests\Admin\UpdateNewsRequest;
use App\Models\News;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Http\RedirectResponse;
use Illuminate\Http\Request;
use Inertia\Inertia;
use Inertia\Response;

class NewsController extends Controller
{
    public function index(Request $request): Response
    {
        $search = $request->string('search')->trim()->value();

        $news = News::query()
            ->when($search !== '', fn (Builder $query) => $query->where(
                fn (Builder $query) => $query
                    ->whereLike('title', "%{$search}%")
                    ->orWhereLike('short_description', "%{$search}%")
            ))
            ->latest()
            ->paginate(15)
            ->withQueryString();

        return Inertia::render('Admin/News/Index', [
            'news' => $news,
            'filters' => ['search' => $search],
        ]);
    }

    public function create(): Response
    {
        return Inertia::render('Admin/News/Create');
    }

    public function store(StoreNewsRequest $request): RedirectResponse
    {
        News::create([
            ...$request->safe()->except('main_image'),
            'main_image' => $request->file('main_image')->store('news'),
        ]);

        return redirect()->route('admin.news.index')
            ->with('success', 'Новоста е креирана.');
    }

    public function edit(News $news): Response
    {
        return Inertia::render('Admin/News/Edit', [
            'news' => $news,
        ]);
    }

    public function update(UpdateNewsRequest $request, News $news): RedirectResponse
    {
        $news->fill($request->safe()->except('main_image'));

        if ($request->hasFile('main_image')) {
            $news->main_image = $request->file('main_image')->store('news');
        }

        $news->save();

        return redirect()->route('admin.news.edit', $news)
            ->with('success', 'Новоста е зачувана.');
    }
}
