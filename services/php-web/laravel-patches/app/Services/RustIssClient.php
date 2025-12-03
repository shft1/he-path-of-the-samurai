<?php

namespace App\Services;

use Illuminate\Support\Facades\Http;

final class RustIssClient
{
    private string $baseUrl;
    private int $timeout;

    public function __construct(?string $baseUrl = null, ?int $timeout = null)
    {
        $this->baseUrl = rtrim($baseUrl ?? env('RUST_BASE', 'http://rust_iss:3000'), '/');
        $this->timeout = $timeout ?? (int) env('RUST_HTTP_TIMEOUT', 10);
    }

    public function lastIss(): array
    {
        return $this->unwrap($this->request('/last'));
    }

    public function trend(array $query = []): array
    {
        return $this->unwrap($this->request('/iss/trend', $query));
    }

    public function osdrList(int $limit): array
    {
        return $this->unwrap($this->request('/osdr/list', ['limit' => $limit]));
    }

    public function raw(string $path, array $query = []): array
    {
        return $this->request($path, $query);
    }

    private function request(string $path, array $query = []): array
    {
        $response = Http::timeout($this->timeout)
            ->retry(2, 200)
            ->acceptJson()
            ->get($this->baseUrl . $path, $query);

        if (!$response->successful()) {
            throw new \RuntimeException(sprintf('RUST_HTTP_%s', $response->status()));
        }

        $json = $response->json();
        return is_array($json) ? $json : [];
    }

    private function unwrap(array $payload): array
    {
        if (($payload['ok'] ?? true) === false) {
            $message = $payload['error']['message'] ?? 'rust upstream error';
            $code    = $payload['error']['code'] ?? 'RUST_UPSTREAM';
            throw new \RuntimeException($code . ': ' . $message);
        }

        if (array_key_exists('data', $payload)) {
            return is_array($payload['data']) ? $payload['data'] : [];
        }

        return $payload;
    }
}

