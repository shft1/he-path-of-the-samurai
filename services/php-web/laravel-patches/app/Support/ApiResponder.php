<?php

namespace App\Support;

use Illuminate\Http\JsonResponse;
use Illuminate\Support\Str;

final class ApiResponder
{
    public static function success(array $data = [], int $status = 200): JsonResponse
    {
        return response()->json([
            'ok'   => true,
            'data' => $data,
        ], $status);
    }

    public static function error(string $code, string $message, int $status = 200, array $extra = []): JsonResponse
    {
        return response()->json([
            'ok'    => false,
            'error' => array_merge([
                'code'     => $code,
                'message'  => $message,
                'trace_id' => (string) Str::uuid(),
            ], $extra),
        ], $status);
    }
}

