<?php

namespace App\Services;

use App\Support\JwstHelper;

final class JwstFeedService
{
    public function __construct(private ?JwstHelper $helper = null)
    {
        $this->helper = $helper ?: new JwstHelper();
    }

    public function feed(array $params): array
    {
        $source = $params['source'] ?? 'jpg';
        $suffix = trim((string) ($params['suffix'] ?? ''));
        $program = trim((string) ($params['program'] ?? ''));
        $instrument = strtoupper(trim((string) ($params['instrument'] ?? '')));
        $page = max(1, (int) ($params['page'] ?? 1));
        $perPage = max(1, min(60, (int) ($params['perPage'] ?? 24)));

        $path = 'all/type/jpg';
        if ($source === 'suffix' && $suffix !== '') {
            $path = 'all/suffix/' . ltrim($suffix, '/');
        } elseif ($source === 'program' && $program !== '') {
            $path = 'program/id/' . rawurlencode($program);
        }

        $response = $this->helper->get($path, ['page' => $page, 'perPage' => $perPage]);
        $list = $response['body'] ?? ($response['data'] ?? (is_array($response) ? $response : []));

        $items = [];
        foreach ($list as $it) {
            if (!is_array($it)) {
                continue;
            }
            $url = $this->resolveImageUrl($it);
            if (!$url) {
                continue;
            }

            $instList = $this->extractInstruments($it);
            if ($instrument && $instList && !in_array($instrument, $instList, true)) {
                continue;
            }

            $items[] = [
                'url'     => $url,
                'obs'     => (string) ($it['observation_id'] ?? $it['observationId'] ?? ''),
                'program' => (string) ($it['program'] ?? ''),
                'suffix'  => (string) ($it['details']['suffix'] ?? $it['suffix'] ?? ''),
                'inst'    => $instList,
                'caption' => trim(
                    (($it['observation_id'] ?? '') ?: ($it['id'] ?? '')) .
                    ' · P' . ($it['program'] ?? '-') .
                    (($it['details']['suffix'] ?? '') ? ' · ' . $it['details']['suffix'] : '') .
                    ($instList ? ' · ' . implode('/', $instList) : '')
                ),
                'link' => $it['location'] ?? $it['url'] ?? $url,
            ];

            if (count($items) >= $perPage) {
                break;
            }
        }

        return [
            'source' => $path,
            'count'  => count($items),
            'items'  => $items,
        ];
    }

    private function resolveImageUrl(array $item): ?string
    {
        $candidates = [
            $item['location'] ?? null,
            $item['url'] ?? null,
            $item['thumbnail'] ?? null,
        ];

        foreach ($candidates as $url) {
            if (is_string($url) && preg_match('~\.(jpg|jpeg|png)(\?.*)?$~i', $url)) {
                return $url;
            }
        }

        return JwstHelper::pickImageUrl($item);
    }

    private function extractInstruments(array $item): array
    {
        $instList = [];
        foreach (($item['details']['instruments'] ?? []) as $instrument) {
            if (is_array($instrument) && !empty($instrument['instrument'])) {
                $instList[] = strtoupper($instrument['instrument']);
            }
        }
        return $instList;
    }
}

