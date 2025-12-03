<?php

namespace App\Http\Controllers;

use App\Services\AstroEventsService;
use App\Support\ApiResponder;
use Illuminate\Http\Request;

class AstroController extends Controller
{
    public function __construct(private AstroEventsService $astro)
    {
    }

    public function events(Request $request)
    {
        $lat  = (float) $request->query('lat', 55.7558);
        $lon  = (float) $request->query('lon', 37.6176);
        $days = max(1, min(30, (int) $request->query('days', 7)));

        $from = now('UTC')->toDateString();
        $to   = now('UTC')->addDays($days)->toDateString();

        try {
            $data = $this->astro->events($lat, $lon, $from, $to);
            return ApiResponder::success($data);
        } catch (\InvalidArgumentException $e) {
            return ApiResponder::error('ASTRO_CONFIG_MISSING', $e->getMessage(), 500);
        } catch (\Throwable $e) {
            return ApiResponder::error('ASTRO_UPSTREAM_FAILED', $e->getMessage());
        }
    }
}
