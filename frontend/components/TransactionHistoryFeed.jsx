import { useState, useEffect, useCallback } from "react";

const EVENT_TYPES = [
  "invoice_created",
  "invoice_paid",
  "invoice_expired",
  "invoice_cancelled",
  "settlement_proposed",
  "settlement_executed",
  "dispute_raised",
  "dispute_resolved",
];

const PAGE_SIZE = 20;

/**
 * Fetches on-chain events from the indexer API.
 * Replace the base URL and query shape to match your actual indexer.
 */
async function fetchEvents({ eventType, address, dateFrom, dateTo, page }) {
  const params = new URLSearchParams({ page, limit: PAGE_SIZE });
  if (eventType) params.set("event_type", eventType);
  if (address) params.set("address", address);
  if (dateFrom) params.set("date_from", dateFrom);
  if (dateTo) params.set("date_to", dateTo);

  const res = await fetch(`/api/events?${params}`);
  if (!res.ok) throw new Error(`Failed to fetch events: ${res.statusText}`);
  return res.json(); // expected: { events: [...], total: number }
}

function EventRow({ event }) {
  return (
    <tr>
      <td>{new Date(event.timestamp * 1000).toLocaleString()}</td>
      <td>
        <span className={`badge badge-${event.event_type}`}>
          {event.event_type}
        </span>
      </td>
      <td title={event.address}>
        {event.address
          ? `${event.address.slice(0, 6)}…${event.address.slice(-4)}`
          : "—"}
      </td>
      <td>{event.amount ?? "—"}</td>
      <td>
        <a
          href={`https://stellar.expert/explorer/testnet/tx/${event.tx_hash}`}
          target="_blank"
          rel="noopener noreferrer"
        >
          {event.tx_hash
            ? `${event.tx_hash.slice(0, 8)}…`
            : "—"}
        </a>
      </td>
    </tr>
  );
}

export default function TransactionHistoryFeed() {
  const [filters, setFilters] = useState({
    eventType: "",
    address: "",
    dateFrom: "",
    dateTo: "",
  });
  const [page, setPage] = useState(1);
  const [events, setEvents] = useState([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await fetchEvents({ ...filters, page });
      setEvents(data.events ?? []);
      setTotal(data.total ?? 0);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [filters, page]);

  useEffect(() => {
    load();
  }, [load]);

  function handleFilterChange(e) {
    const { name, value } = e.target;
    setFilters((prev) => ({ ...prev, [name]: value }));
    setPage(1);
  }

  const totalPages = Math.ceil(total / PAGE_SIZE);

  return (
    <div className="tx-history">
      <h2>Transaction History</h2>

      {/* Filters */}
      <div className="tx-history__filters">
        <select
          name="eventType"
          value={filters.eventType}
          onChange={handleFilterChange}
          aria-label="Filter by event type"
        >
          <option value="">All event types</option>
          {EVENT_TYPES.map((t) => (
            <option key={t} value={t}>
              {t}
            </option>
          ))}
        </select>

        <input
          type="text"
          name="address"
          placeholder="Filter by address"
          value={filters.address}
          onChange={handleFilterChange}
          aria-label="Filter by address"
        />

        <label>
          From
          <input
            type="date"
            name="dateFrom"
            value={filters.dateFrom}
            onChange={handleFilterChange}
            aria-label="From date"
          />
        </label>

        <label>
          To
          <input
            type="date"
            name="dateTo"
            value={filters.dateTo}
            onChange={handleFilterChange}
            aria-label="To date"
          />
        </label>

        <button onClick={load} disabled={loading}>
          {loading ? "Loading…" : "Refresh"}
        </button>
      </div>

      {/* Error */}
      {error && <p className="tx-history__error" role="alert">{error}</p>}

      {/* Table */}
      <table className="tx-history__table" aria-label="Transaction history">
        <thead>
          <tr>
            <th>Time</th>
            <th>Event</th>
            <th>Address</th>
            <th>Amount</th>
            <th>Tx Hash</th>
          </tr>
        </thead>
        <tbody>
          {events.length === 0 && !loading ? (
            <tr>
              <td colSpan={5} style={{ textAlign: "center" }}>
                No events found.
              </td>
            </tr>
          ) : (
            events.map((ev, i) => <EventRow key={`${ev.tx_hash}-${i}`} event={ev} />)
          )}
        </tbody>
      </table>

      {/* Pagination */}
      {totalPages > 1 && (
        <div className="tx-history__pagination" role="navigation" aria-label="Pagination">
          <button
            onClick={() => setPage((p) => Math.max(1, p - 1))}
            disabled={page <= 1}
            aria-label="Previous page"
          >
            ← Prev
          </button>
          <span>
            Page {page} of {totalPages}
          </span>
          <button
            onClick={() => setPage((p) => Math.min(totalPages, p + 1))}
            disabled={page >= totalPages}
            aria-label="Next page"
          >
            Next →
          </button>
        </div>
      )}
    </div>
  );
}
