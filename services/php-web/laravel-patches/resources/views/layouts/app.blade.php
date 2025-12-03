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
      --space-dark: #0d0d0d;
      --space-purple: #2d1b4e;
      --space-accent: #ff6b35;
      --space-glow: #ffd700;
      --space-teal: #00d4aa;
    }
    body {
      background: linear-gradient(135deg, var(--space-dark) 0%, #1a1a2e 50%, var(--space-purple) 100%);
      min-height: 100vh;
      font-family: 'Space Grotesk', sans-serif;
    }
    .space-header {
      background: linear-gradient(90deg, rgba(13,13,13,0.95) 0%, rgba(45,27,78,0.9) 100%);
      border-bottom: 2px solid var(--space-accent);
      box-shadow: 0 4px 30px rgba(255,107,53,0.2);
      backdrop-filter: blur(10px);
    }
    .space-brand {
      font-family: 'Orbitron', monospace;
      font-weight: 700;
      font-size: 1.5rem;
      color: var(--space-glow) !important;
      text-shadow: 0 0 20px rgba(255,215,0,0.5);
      letter-spacing: 2px;
    }
    .space-brand:hover {
      color: var(--space-accent) !important;
      text-shadow: 0 0 30px rgba(255,107,53,0.7);
    }
    .space-nav-link {
      font-family: 'Space Grotesk', sans-serif;
      font-weight: 500;
      color: #e0e0e0 !important;
      padding: 0.5rem 1.2rem !important;
      margin: 0 0.25rem;
      border-radius: 20px;
      transition: all 0.3s ease;
      position: relative;
    }
    .space-nav-link:hover {
      color: var(--space-glow) !important;
      background: rgba(255,215,0,0.1);
    }
    .space-nav-link.active {
      background: linear-gradient(135deg, var(--space-accent), #ff8c5a);
      color: #fff !important;
    }
    .nav-icon {
      margin-right: 6px;
    }
    #map{height:340px; border-radius: 12px;}
    .card {
      background: rgba(30,30,50,0.8);
      border: 1px solid rgba(255,107,53,0.2);
      backdrop-filter: blur(5px);
    }
    .card-title, .card-header {
      color: var(--space-glow);
      font-family: 'Orbitron', monospace;
    }
    .text-muted { color: #a0a0a0 !important; }
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
