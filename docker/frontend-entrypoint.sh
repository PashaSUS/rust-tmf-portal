#!/bin/sh
# Write runtime config from environment variables
cat > /app/dist/runtime-config.js <<EOF
window.__RUNTIME_CONFIG = {
  apiUrl: "${API_URL:-}",
};
EOF

exec serve -s dist -l 3000
