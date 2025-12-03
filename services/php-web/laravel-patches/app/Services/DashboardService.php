<?php

namespace App\Services;

final class DashboardService
{
    public function __construct(private RustIssClient $rust)
    {
    }

    public function viewData(): array
    {
        $iss = $this->safeFetch(fn () => $this->rust->lastIss());

        $payload = $iss['payload'] ?? [];

        return [
            'iss'                    => $iss,
            'trend'                  => [],
            'jw_gallery'             => [],
            'jw_observation_raw'     => [],
            'jw_observation_summary' => [],
            'jw_observation_images'  => [],
            'jw_observation_files'   => [],
            'metrics'                => [
                'iss_speed' => $payload['velocity'] ?? null,
                'iss_alt'   => $payload['altitude'] ?? null,
                'neo_total' => 0,
            ],
        ];
    }

    private function safeFetch(callable $callback): array
    {
        try {
            $result = $callback();
            return is_array($result) ? $result : [];
        } catch (\Throwable $e) {
            return ['error' => $e->getMessage()];
        }
    }
}

