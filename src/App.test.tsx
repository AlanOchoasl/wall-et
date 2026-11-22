import { beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";

import { App } from "./App";
import { getAppStatus } from "./shared/api";

vi.mock("./shared/api", () => ({
  getAppStatus: vi.fn(),
}));

const mockedGetAppStatus = vi.mocked(getAppStatus);

describe("App", () => {
  beforeEach(() => {
    mockedGetAppStatus.mockReset();
  });

  it("muestra el título de la aplicación", () => {
    mockedGetAppStatus.mockResolvedValue({
      appName: "App de Finanzas",
      version: "0.1.0",
      databaseReady: true,
      schemaVersion: 1,
      currencyCode: "ARS",
    });

    render(<App />);

    expect(
      screen.getByRole("heading", { name: "App de Finanzas" }),
    ).toBeInTheDocument();
  });

  it("muestra el estado informado por el backend", async () => {
    mockedGetAppStatus.mockResolvedValue({
      appName: "App de Finanzas",
      version: "0.1.0",
      databaseReady: true,
      schemaVersion: 1,
      currencyCode: "ARS",
    });

    render(<App />);

    expect(await screen.findByText("lista (esquema 1)")).toBeInTheDocument();
    expect(screen.getByText("ARS")).toBeInTheDocument();
  });

  it("muestra un mensaje cuando el comando falla", async () => {
    mockedGetAppStatus.mockRejectedValue(new Error("sin conexión"));

    render(<App />);

    expect(await screen.findByRole("alert")).toHaveTextContent("sin conexión");
  });
});
