<!doctype html>
<html lang="ru">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Space Dashboard</title>
  <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
  <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"/>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link href="https://fonts.googleapis.com/css2?family=Orbitron:wght@400;700&family=Space+Grotesk:wght@400;500;600&display=swap" rel="stylesheet">
  <style>
    :root {
      --space-light: #f8f9fa;
      --space-cream: #fefefe;
      --header-bg: #1e293b;
      --accent-orange: #f97316;
      --accent-amber: #f59e0b;
      --text-dark: #1e293b;
    }
    body {
      background: linear-gradient(180deg, var(--space-light) 0%, #e2e8f0 100%);
      min-height: 100vh;
      font-family: 'Space Grotesk', sans-serif;
      color: var(--text-dark);
    }
    .space-header {
      background: var(--header-bg);
      border-bottom: 3px solid var(--accent-orange);
      box-shadow: 0 4px 20px rgba(0,0,0,0.15);
    }
    .space-brand {
      font-family: 'Orbitron', monospace;
      font-weight: 700;
      font-size: 1.4rem;
      color: #fff !important;
      letter-spacing: 2px;
    }
    .space-brand:hover {
      color: var(--accent-amber) !important;
    }
    .space-nav-link {
      font-family: 'Space Grotesk', sans-serif;
      font-weight: 500;
      color: #cbd5e1 !important;
      padding: 0.5rem 1.2rem !important;
      margin: 0 0.25rem;
      border-radius: 20px;
      transition: all 0.3s ease;
    }
    .space-nav-link:hover {
      color: #fff !important;
      background: rgba(255,255,255,0.1);
    }
    .space-nav-link.active {
      background: linear-gradient(135deg, var(--accent-orange), var(--accent-amber));
      color: #fff !important;
    }
    .nav-icon { margin-right: 6px; }
    #map { height: 340px; border-radius: 12px; }
    .card {
      background: var(--space-cream);
      border: 1px solid #e2e8f0;
      box-shadow: 0 2px 8px rgba(0,0,0,0.06);
    }
    .card-title, .card-header {
      color: var(--text-dark);
      font-family: 'Orbitron', monospace;
      font-weight: 600;
    }
    .card-header {
      background: linear-gradient(90deg, #f1f5f9, #e2e8f0);
      border-bottom: 2px solid var(--accent-orange);
    }
    .container { max-width: 1400px; }
  </style>
  <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
  <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
</head>
<body>
<nav class="navbar navbar-expand-lg space-header sticky-top py-3 mb-4">
  <div class="container">
    <a class="navbar-brand space-brand" href="/dashboard">
      <span class="nav-icon">🚀</span>SPACE HUB
    </a>
    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navMenu">
      <span class="navbar-toggler-icon"></span>
    </button>
    <div class="collapse navbar-collapse" id="navMenu">
      <ul class="navbar-nav ms-auto">
        <li class="nav-item">
          <a class="nav-link space-nav-link {{ request()->is('dashboard*') ? 'active' : '' }}" href="/dashboard">
            <span class="nav-icon">📊</span>Dashboard
          </a>
        </li>
        <li class="nav-item">
          <a class="nav-link space-nav-link {{ request()->is('iss*') ? 'active' : '' }}" href="/dashboard">
            <span class="nav-icon">🛰️</span>ISS
          </a>
        </li>
        <li class="nav-item">
          <a class="nav-link space-nav-link {{ request()->is('osdr*') ? 'active' : '' }}" href="/osdr">
            <span class="nav-icon">🔬</span>OSDR
          </a>
        </li>
      </ul>
    </div>
  </div>
</nav>
@yield('content')
<script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
</body>
</html>
