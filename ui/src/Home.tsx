// The organisation at a glance: members by standing, this year's giving, grants with reports owed,
// and volunteers' approved hours. Each figure opens its list.

import { useQuery } from "@tanstack/react-query";
import { Link } from "react-router";
import { api, docPath } from "@ferrum/client";
import { ErrorState, PageLoading, useShellTitle } from "@ferrum/kit";

interface Row {
  [k: string]: unknown;
}

const rows = (entity: string, filters: unknown[], fields: string) =>
  api.get<{ rows: Row[] }>(docPath(entity), { filters: JSON.stringify(filters), fields, limit: 1000 }).then((r) => r.rows);

/** Sum of a field, as the list returns it (decimals come as strings). */
export const sum = (list: Row[], field: string) => list.reduce((acc, r) => acc + Number(r[field] ?? 0), 0);

export function Home() {
  useShellTitle("Nonprofit");
  const year = new Date().getFullYear();
  const q = useQuery({
    queryKey: ["nonprofit", "home", year],
    queryFn: async () => {
      const [members, donations, grants, volunteers] = await Promise.all([
        rows("Member", [], "status"),
        rows("Donation", [["docstatus", "eq", 1], ["date", "gte", `${year}-01-01`]], "amount"),
        rows("Grant", [["status", "in", ["Submitted", "Awarded"]]], "status,requested,awarded"),
        rows("Volunteer", [["active", "eq", true]], "hours"),
      ]);
      return { members, donations, grants, volunteers };
    },
  });
  if (q.isPending) return <PageLoading />;
  if (q.isError) return <ErrorState error={q.error} onRetry={() => void q.refetch()} />;
  const { members, donations, grants, volunteers } = q.data;
  const count = (status: string) => members.filter((m) => m.status === status).length;
  const figures: { label: string; value: string; to: string }[] = [
    { label: "Active members", value: String(count("Active")), to: "/e/Member" },
    { label: "Lapsed members", value: String(count("Lapsed")), to: "/e/Member" },
    { label: `Given in ${year}`, value: sum(donations, "amount").toLocaleString(undefined, { maximumFractionDigits: 2 }), to: "/e/Donation" },
    { label: "Grants awaiting or awarded", value: String(grants.length), to: "/e/Grant" },
    { label: "Volunteers' approved hours", value: sum(volunteers, "hours").toLocaleString(), to: "/e/Volunteer" },
  ];
  return (
    <div className="np-page">
      <div className="np-columns">
        {figures.map((f) => (
          <Link key={f.label} to={f.to} className="card card-pad stack">
            <span className="muted">{f.label}</span>
            <strong style={{ fontSize: 28 }}>{f.value}</strong>
          </Link>
        ))}
      </div>
    </div>
  );
}
