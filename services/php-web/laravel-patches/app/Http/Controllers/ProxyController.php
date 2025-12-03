<?php

namespace App\Http\Controllers;

use App\Services\RustIssClient;
use App\Support\ApiResponder;
use Illuminate\Http\Request;

class ProxyController extends Controller
{
    public function __construct(private RustIssClient $rust)
    {
    }

    public function last()
    {
        return $this->forward('/last');
    }

    public function trend(Request $request)
    {
        return $this->forward('/iss/trend', $request->query());
    }

    private function forward(string $path, array $query = [])
    {
        try {
            $payload = $this->rust->raw($path, $query);
            return response()->json($payload);
        } catch (\Throwable $e) {
            return ApiResponder::error('RUST_PROXY_FAILED', $e->getMessage());
        }
    }
}
