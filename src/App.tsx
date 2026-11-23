import { useEffect, useState } from "react";

import { ServicesScreen } from "./features/services/ServicesScreen";
import { getAppStatus, type AppStatus } from "./shared/api";

export function App() {
  const [status, setStatus] = useState<AppStatus | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let active = true;

    getAppStatus()
      .then((value) => {
        if (active) {
          setStatus(value);
        }
      })
      .catch((reason: unknown) => {
        if (active) {
          setError(
            reason instanceof Error
              ? reason.message
              : "No se pudo obtener el estado de la aplicación.",
          );
        }
      });

    return () => {
      active = false;
    };
  }, []);

  return (
    <main className="app-shell">
      <h1>App de Finanzas</h1>

      {error ? <p role="alert">{error}</p> : null}

      {status ? (
        <dl className="app-status">
          <div>
            <dt>Versión</dt>
            <dd>{status.version}</dd>
          </div>
          <div>
            <dt>Base de datos</dt>
            <dd>
              {status.databaseReady
                ? `lista (esquema ${status.schemaVersion})`
                : "no disponible"}
            </dd>
          </div>
          <div>
            <dt>Moneda</dt>
            <dd>{status.currencyCode}</dd>
          </div>
        </dl>
      ) : null}

      {!status && !error ? <p>Cargando estado…</p> : null}

      <ServicesScreen />
    </main>
  );
}
