<?php

namespace App\Services;

use Illuminate\Support\Facades\Http;

final class AstroEventsService
{
    private string $appId;
    private string $secret;
    private int $timeout;

    public function __construct(?string $appId = null, ?string $secret = null, ?int $timeout = null)
    {
        $this->appId  = $appId ?? env('ASTRO_APP_ID', '');
        $this->secret = $secret ?? env('ASTRO_APP_SECRET', '');
        $this->timeout = $timeout ?? (int) env('ASTRO_HTTP_TIMEOUT', 25);
    }

    public function events(float $lat, float $lon, string $from, string $to): array
    {
        if ($this->appId === '' || $this->secret === '') {
            throw new \InvalidArgumentException('ASTRO_APP_ID/ASTRO_APP_SECRET are required');
        }

        $response = Http::timeout($this->timeout)
            ->withBasicAuth($this->appId, $this->secret)
            ->acceptJson()
            ->get('https://api.astronomyapi.com/api/v2/bodies/events', [
                'latitude'  => $lat,
                'longitude' => $lon,
                'from'      => $from,
                'to'        => $to,
            ]);

        if (!$response->successful()) {
            throw new \RuntimeException(sprintf('ASTRO_HTTP_%s', $response->status()));
        }

        $json = $response->json();
        return is_array($json) ? $json : [];
    }
}

