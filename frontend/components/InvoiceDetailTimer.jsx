import { useState, useEffect, useRef } from "react";

/**
 * Parses seconds remaining into { days, hours, minutes, seconds }.
 */
function parseCountdown(totalSeconds) {
  const s = Math.max(0, totalSeconds);
  return {
    days: Math.floor(s / 86400),
    hours: Math.floor((s % 86400) / 3600),
    minutes: Math.floor((s % 3600) / 60),
    seconds: s % 60,
  };
}

function pad(n) {
  return String(n).padStart(2, "0");
}

/**
 * CountdownTimer
 *
 * Props:
 *   expiresAt  {number}   Unix timestamp (seconds) of the invoice expiry.
 *   onExpired  {function} Callback fired when the countdown reaches zero.
 *                         The parent should use this to refresh the invoice status.
 */
export function CountdownTimer({ expiresAt, onExpired }) {
  const [remaining, setRemaining] = useState(() =>
    Math.max(0, expiresAt - Math.floor(Date.now() / 1000))
  );
  const onExpiredRef = useRef(onExpired);
  onExpiredRef.current = onExpired;

  useEffect(() => {
    if (remaining <= 0) return;

    const id = setInterval(() => {
      const now = Math.floor(Date.now() / 1000);
      const diff = expiresAt - now;
      if (diff <= 0) {
        setRemaining(0);
        clearInterval(id);
        onExpiredRef.current?.();
      } else {
        setRemaining(diff);
      }
    }, 1000);

    return () => clearInterval(id);
  }, [expiresAt]); // expiresAt is stable; re-run only if the invoice changes

  if (remaining <= 0) return null; // caller renders the Expired badge instead

  const { days, hours, minutes, seconds } = parseCountdown(remaining);

  return (
    <div className="countdown" role="timer" aria-live="off" aria-label="Time remaining">
      {days > 0 && (
        <span className="countdown__segment">
          <strong>{days}</strong>
          <small>d</small>
        </span>
      )}
      <span className="countdown__segment">
        <strong>{pad(hours)}</strong>
        <small>h</small>
      </span>
      <span className="countdown__segment">
        <strong>{pad(minutes)}</strong>
        <small>m</small>
      </span>
      <span className="countdown__segment">
        <strong>{pad(seconds)}</strong>
        <small>s</small>
      </span>
    </div>
  );
}

/**
 * InvoiceDetailTimer
 *
 * Wraps CountdownTimer with invoice-status auto-refresh logic.
 *
 * Props:
 *   invoice    {object}   Invoice object with at least { expires_at, status }.
 *   onRefresh  {function} Async function that re-fetches and updates the invoice.
 */
export default function InvoiceDetailTimer({ invoice, onRefresh }) {
  const isExpired =
    invoice.status === "Expired" ||
    invoice.expires_at <= Math.floor(Date.now() / 1000);

  async function handleExpired() {
    try {
      await onRefresh?.();
    } catch {
      // silently ignore refresh errors — the UI can retry on next interaction
    }
  }

  if (isExpired) {
    return (
      <span className="badge badge--expired" aria-label="Invoice expired">
        Expired
      </span>
    );
  }

  return (
    <CountdownTimer expiresAt={invoice.expires_at} onExpired={handleExpired} />
  );
}
