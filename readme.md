<code>
Browser (localhost:8001)
         ↓
┌─────────────────────────────────┐
│   ingress/src/main.rs           │  ← Entry point, request routing
│   └── catch_all()               │      - Serves frontend (index.html)
│       └── index()               │      - Routes API calls
└─────────────────────────────────┘
         ↓ (Frontend makes API call)
┌─────────────────────────────────┐
│   to_do/networking/actix_server │  ← HTTP handlers (Actix Web)
│   └── api/basic_actions/get.rs  │      - get_all()
└─────────────────────────────────┘
         ↓
┌─────────────────────────────────┐
│   to_do/core/src/api/           │  ← Business logic layer
│   └── basic_actions/get.rs      │      - get_all_core()
└─────────────────────────────────┘
         ↓
┌─────────────────────────────────┐
│   to_do/dal/src/json_file.rs    │  ← Data Access Layer
│   └── get_all()                 │      - Reads tasks.json
└─────────────────────────────────┘
         ↓
    tasks.json file
</code>