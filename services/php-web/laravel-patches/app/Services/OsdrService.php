<?php

namespace App\Services;

final class OsdrService
{
    public function __construct(private RustIssClient $rust)
    {
    }

    public function list(int $limit): array
    {
        $response = $this->rust->osdrList($limit);
        $items    = $response['items'] ?? [];
        return $this->flatten($items);
    }

    private function flatten(array $items): array
    {
        $out = [];
        foreach ($items as $row) {
            $raw = $row['raw'] ?? [];
            if (is_array($raw) && $this->looksOsdrDict($raw)) {
                foreach ($raw as $k => $v) {
                    if (!is_array($v)) {
                        continue;
                    }
                    $rest  = $v['REST_URL'] ?? $v['rest_url'] ?? $v['rest'] ?? null;
                    $title = $v['title'] ?? $v['name'] ?? null;
                    if (!$title && is_string($rest)) {
                        $title = basename(rtrim($rest, '/'));
                    }
                    $out[] = [
                        'id'          => $row['id'] ?? null,
                        'dataset_id'  => $k,
                        'title'       => $title,
                        'status'      => $row['status'] ?? null,
                        'updated_at'  => $row['updated_at'] ?? null,
                        'inserted_at' => $row['inserted_at'] ?? null,
                        'rest_url'    => $rest,
                        'raw'         => $v,
                    ];
                }
            } else {
                $row['rest_url'] = is_array($raw)
                    ? ($raw['REST_URL'] ?? $raw['rest_url'] ?? null)
                    : null;
                $out[] = $row;
            }
        }
        return $out;
    }

    private function looksOsdrDict(array $raw): bool
    {
        foreach ($raw as $k => $v) {
            if (is_string($k) && str_starts_with($k, 'OSD-')) {
                return true;
            }
            if (is_array($v) && (isset($v['REST_URL']) || isset($v['rest_url']))) {
                return true;
            }
        }
        return false;
    }
}

