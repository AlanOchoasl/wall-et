import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";

import {
  createInvestment,
  createService,
  deleteService,
  freezeService,
  getServiceTotals,
  listActiveServices,
  listFrozenServices,
  listInvestmentsByMonth,
  reactivateService,
  updateInvestment,
  updateService,
  type CreateInvestmentInputDto,
  type CreateServiceInputDto,
  type InvestmentSummaryDto,
  type ServiceRecordDto,
  type ServiceTotalsDto,
  type UpdateInvestmentInputDto,
  type UpdateServiceInputDto,
} from "./api";
import { CommandError } from "./commands";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

const mockedInvoke = vi.mocked(invoke);

const record: ServiceRecordDto = {
  id: "00000000-0000-4000-8000-000000000001",
  name: "Luz",
  totalCost: {
    minorUnits: "1000000",
    currencyCode: "ARS",
  },
  myShare: {
    numeratorMinor: "10000",
    denominator: "3",
    currencyCode: "ARS",
  },
  sharedPeopleCount: 3,
  notes: null,
  status: "active",
  createdAt: "2026-11-23T12:00:00.000Z",
  updatedAt: "2026-11-23T12:00:00.000Z",
};

const createInput: CreateServiceInputDto = {
  name: "Luz",
  totalCost: "10000,00",
  currencyCode: "ARS",
  sharedPeopleCount: 3,
  notes: null,
};

const updateInput: UpdateServiceInputDto = {
  name: "Luz",
  totalCost: "12000,00",
  sharedPeopleCount: 4,
  notes: null,
};

describe("servicios API", () => {
  beforeEach(() => {
    mockedInvoke.mockReset();
  });

  it("createService invoca create_service con el input", async () => {
    mockedInvoke.mockResolvedValue(record);

    await expect(createService(createInput)).resolves.toEqual(record);
    expect(mockedInvoke).toHaveBeenCalledWith("create_service", {
      input: createInput,
    });
  });

  it("updateService invoca update_service con id e input", async () => {
    mockedInvoke.mockResolvedValue(record);

    await expect(updateService(record.id, updateInput)).resolves.toEqual(
      record,
    );
    expect(mockedInvoke).toHaveBeenCalledWith("update_service", {
      id: record.id,
      input: updateInput,
    });
  });

  it("deleteService invoca delete_service con la confirmación", async () => {
    mockedInvoke.mockResolvedValue({ outcome: "cancelled", id: record.id });

    await expect(deleteService(record.id, false)).resolves.toEqual({
      outcome: "cancelled",
      id: record.id,
    });
    expect(mockedInvoke).toHaveBeenCalledWith("delete_service", {
      id: record.id,
      confirmed: false,
    });
  });

  it("freezeService invoca freeze_service con el id", async () => {
    mockedInvoke.mockResolvedValue({ outcome: "already_frozen" });

    await expect(freezeService(record.id)).resolves.toEqual({
      outcome: "already_frozen",
    });
    expect(mockedInvoke).toHaveBeenCalledWith("freeze_service", {
      id: record.id,
    });
  });

  it("reactivateService invoca reactivate_service con el id", async () => {
    mockedInvoke.mockResolvedValue({ outcome: "reactivated" });

    await expect(reactivateService(record.id)).resolves.toEqual({
      outcome: "reactivated",
    });
    expect(mockedInvoke).toHaveBeenCalledWith("reactivate_service", {
      id: record.id,
    });
  });

  it("listFrozenServices invoca list_frozen_services sin argumentos", async () => {
    mockedInvoke.mockResolvedValue([record]);

    await expect(listFrozenServices()).resolves.toEqual([record]);
    expect(mockedInvoke).toHaveBeenCalledWith("list_frozen_services", undefined);
  });

  it("listActiveServices invoca list_active_services sin argumentos", async () => {
    mockedInvoke.mockResolvedValue([record]);

    await expect(listActiveServices()).resolves.toEqual([record]);
    expect(mockedInvoke).toHaveBeenCalledWith("list_active_services", undefined);
  });

  it("getServiceTotals conserva importes y fracciones como cadenas", async () => {
    const totals: ServiceTotalsDto = {
      ars: {
        totalCost: { minorUnits: "20000", currencyCode: "ARS" },
        userShare: {
          numeratorMinor: "20000",
          denominator: "3",
          currencyCode: "ARS",
        },
      },
      usd: {
        totalCost: { minorUnits: "0", currencyCode: "USD" },
        userShare: {
          numeratorMinor: "0",
          denominator: "1",
          currencyCode: "USD",
        },
      },
    };
    mockedInvoke.mockResolvedValue(totals);

    await expect(getServiceTotals()).resolves.toEqual(totals);
    expect(mockedInvoke).toHaveBeenCalledWith("get_service_totals", undefined);
  });

  it("propaga CommandError con código, mensaje y campo", async () => {
    mockedInvoke.mockRejectedValue({
      code: "validation",
      message: "El costo no es válido.",
      field: "totalCost",
    });

    await expect(createService(createInput)).rejects.toMatchObject({
      name: "CommandError",
      code: "validation",
      message: "El costo no es válido.",
      field: "totalCost",
    });
    await expect(createService(createInput)).rejects.toBeInstanceOf(
      CommandError,
    );
  });
});

describe("inversiones API", () => {
  const investmentRecord: InvestmentSummaryDto = {
    record: {
      id: "00000000-0000-4000-8000-000000000001",
      month: "2026-03",
      currencyCode: "ARS",
      capital: { minorUnits: "36500", currencyCode: "ARS" },
      tna: "100,00",
      termDays: 365,
      taxes: { minorUnits: "6500", currencyCode: "ARS" },
      createdAt: "2026-11-23T12:00:00.000Z",
      updatedAt: "2026-11-23T12:00:00.000Z",
    },
    derived: {
      monthlyExpenses: {
        numeratorMinor: "10000",
        denominator: "1",
        currencyCode: "ARS",
      },
      grossYield: {
        definitive: {
          status: "no_disponible",
          reason: "pending_gross_yield_formula",
          value: null,
        },
        provisional: {
          status: "provisional",
          reason: "approved_demo_pv01",
          value: {
            numeratorMinor: "36500",
            denominator: "1",
            currencyCode: "ARS",
          },
        },
      },
      netYield: {
        definitive: {
          status: "no_disponible",
          reason: "pending_net_yield_treatment",
          value: null,
        },
        provisional: {
          status: "provisional",
          reason: "approved_demo_pv02",
          value: {
            numeratorMinor: "30000",
            denominator: "1",
            currencyCode: "ARS",
          },
        },
      },
      coverage: {
        definitive: {
          status: "no_disponible",
          reason: "pending_net_yield",
          value: null,
        },
        provisional: {
          status: "provisional",
          reason: "approved_demo_pv03",
          value: {
            outcome: "deficit",
            amount: {
              numeratorMinor: "5000",
              denominator: "1",
              currencyCode: "ARS",
            },
          },
        },
      },
    },
  };

  const createInput: CreateInvestmentInputDto = {
    month: "2026-03",
    currencyCode: "ARS",
    capital: "365,00",
    tna: "100",
    termDays: "365",
    taxes: "65,00",
  };

  const updateInput: UpdateInvestmentInputDto = {
    month: "2026-04",
    capital: "730,00",
    tna: "100",
    termDays: "365",
    taxes: "65,00",
  };

  beforeEach(() => {
    mockedInvoke.mockReset();
  });

  it("createInvestment invoca create_investment y conserva derivados", async () => {
    mockedInvoke.mockResolvedValue(investmentRecord);

    await expect(createInvestment(createInput)).resolves.toEqual(
      investmentRecord,
    );
    expect(mockedInvoke).toHaveBeenCalledWith("create_investment", {
      input: createInput,
    });
  });

  it("updateInvestment invoca update_investment con id e input", async () => {
    mockedInvoke.mockResolvedValue(investmentRecord);

    await expect(
      updateInvestment(investmentRecord.record.id, updateInput),
    ).resolves.toEqual(investmentRecord);
    expect(mockedInvoke).toHaveBeenCalledWith("update_investment", {
      id: investmentRecord.record.id,
      input: updateInput,
    });
  });

  it("listInvestmentsByMonth invoca list_investments_by_month con el mes", async () => {
    mockedInvoke.mockResolvedValue([investmentRecord]);

    await expect(listInvestmentsByMonth("2026-03")).resolves.toEqual([
      investmentRecord,
    ]);
    expect(mockedInvoke).toHaveBeenCalledWith("list_investments_by_month", {
      month: "2026-03",
    });
  });

  it("propaga CommandError con el campo de validación de inversiones", async () => {
    mockedInvoke.mockRejectedValue({
      code: "validation",
      message: "La moneda debe ser ARS o USD.",
      field: "currencyCode",
    });

    await expect(createInvestment(createInput)).rejects.toMatchObject({
      name: "CommandError",
      code: "validation",
      message: "La moneda debe ser ARS o USD.",
      field: "currencyCode",
    });
  });
});
