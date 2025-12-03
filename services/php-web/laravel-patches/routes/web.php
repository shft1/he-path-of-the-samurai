<?php

use App\Http\Controllers\AstroController;
use App\Http\Controllers\CmsController;
use App\Http\Controllers\DashboardController;
use App\Http\Controllers\OsdrController;
use App\Http\Controllers\ProxyController;
use Illuminate\Support\Facades\Route;

Route::get('/', fn () => redirect('/dashboard'));

Route::get('/dashboard', [DashboardController::class, 'index']);
Route::get('/osdr', [OsdrController::class, 'index']);

Route::get('/api/iss/last', [ProxyController::class, 'last']);
Route::get('/api/iss/trend', [ProxyController::class, 'trend']);

Route::get('/api/jwst/feed', [DashboardController::class, 'jwstFeed']);
Route::get('/api/astro/events', [AstroController::class, 'events']);

Route::get('/page/{slug}', [CmsController::class, 'page']);
