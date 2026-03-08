<?php

require_once __DIR__ . '/vendor/autoload.php';

// ── Test HTML payloads ──────────────────────────────────────────────

$small = '<p>Hello <script>alert("xss")</script> <b>world</b></p>';

$medium = str_repeat(
    '<div class="container"><h2>Section</h2>'
    . '<p>Some text with <a href="https://example.com" onclick="steal()">a link</a> and <img src="photo.jpg" onerror="hack()"> images.</p>'
    . '<script>document.cookie</script>'
    . '<style>body { display: none }</style>'
    . '<table><tr><td>Data</td></tr></table>'
    . '<iframe src="evil.com"></iframe>'
    . '</div>',
    20
);

$large = str_repeat($medium, 50); // ~1MB of HTML

$payloads = [
    'small (~56 bytes)'   => $small,
    'medium (~5 KB)'      => $medium,
    'large (~250 KB)'     => $large,
];

// ── Benchmark config ────────────────────────────────────────────────

$iterations = [
    'small (~56 bytes)'   => 10000,
    'medium (~5 KB)'      => 1000,
    'large (~250 KB)'     => 50,
];

// ── HTMLPurifier setup ──────────────────────────────────────────────

$purifierConfig = HTMLPurifier_Config::createDefault();
$cacheDir = __DIR__ . '/cache/htmlpurifier';
if (!is_dir($cacheDir)) {
    mkdir($cacheDir, 0755, true);
}
$purifierConfig->set('Cache.SerializerPath', $cacheDir);
$purifier = new HTMLPurifier($purifierConfig);

// ── Warmup ──────────────────────────────────────────────────────────

$purifier->purify($small);
sanitize_html($small);

// ── Run benchmarks ──────────────────────────────────────────────────

echo "HTML Sanitization Benchmark\n";
echo str_repeat('=', 70) . "\n";
printf("%-20s %15s %15s %15s\n", 'Payload', 'HTMLPurifier', 'ammonia', 'Speedup');
echo str_repeat('-', 70) . "\n";

foreach ($payloads as $label => $html) {
    $n = $iterations[$label];
    $size = strlen($html);

    // Benchmark HTMLPurifier
    $start = hrtime(true);
    for ($i = 0; $i < $n; $i++) {
        $purifier->purify($html);
    }
    $purifierTime = (hrtime(true) - $start) / 1e6; // ms

    // Benchmark sanitize_rs (ammonia)
    $start = hrtime(true);
    for ($i = 0; $i < $n; $i++) {
        sanitize_html($html);
    }
    $rsTime = (hrtime(true) - $start) / 1e6; // ms

    $speedup = $purifierTime / $rsTime;
    $purifierAvg = $purifierTime / $n;
    $rsAvg = $rsTime / $n;

    printf(
        "%-20s %12.2f ms %12.2f ms %12.1fx\n",
        $label,
        $purifierAvg,
        $rsAvg,
        $speedup
    );
}

echo str_repeat('=', 70) . "\n";

// ── Correctness spot-check ──────────────────────────────────────────

echo "\nCorrectness check (small payload):\n";
echo "  Input:          " . $small . "\n";
echo "  HTMLPurifier:   " . $purifier->purify($small) . "\n";
echo "  ammonia:        " . sanitize_html($small) . "\n";
