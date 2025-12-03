<?php

namespace App\Http\Controllers;

use App\Services\DashboardService;
use App\Services\JwstFeedService;
use App\Support\ApiResponder;
use Illuminate\Http\Request;

class DashboardController extends Controller
{
    public function __construct(private DashboardService $dashboard)
    {
    }

    public function index()
    {
        return view('dashboard', $this->dashboard->viewData());
    }

    public function jwstFeed(Request $request, JwstFeedService $jwst)
    {
        try {
            $data = $jwst->feed($request->all());
            return ApiResponder::success($data);
        } catch (\Throwable $e) {
            return ApiResponder::error('JWST_FEED_FAILED', $e->getMessage());
        }
    }
}
