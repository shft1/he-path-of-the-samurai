<?php

namespace App\Http\Controllers;

use App\Services\OsdrService;
use Illuminate\Http\Request;

class OsdrController extends Controller
{
    public function __construct(private OsdrService $service)
    {
    }

    public function index(Request $request)
    {
        $limit = (int) $request->query('limit', 20);
        $limit = max(1, min(200, $limit));

        $items = $this->service->list($limit);
        $endpoint = rtrim(env('RUST_BASE', 'http://rust_iss:3000'), '/') . '/osdr/list?limit=' . $limit;

        return view('osdr', [
            'items' => $items,
            'src'   => $endpoint,
        ]);
    }
}
