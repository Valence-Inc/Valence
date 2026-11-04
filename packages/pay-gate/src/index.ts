/**
 * Pay-gate middleware for worker endpoints.
 *
 * v1: Internal stub that always approves requests.
 * Future: Validates x402 payment proofs or on-chain pre-payment.
 */

// TODO(contrib): Real x402 / payment-proof validation
// TODO(contrib): Rate limiting and abuse prevention

export interface PayGateConfig {
  /** Whether to enforce payment (false in v1). */
  enforce: boolean;
}

/**
 * Stub middleware factory. Returns a pass-through in v1.
 * Designed for Hono — signature matches MiddlewareHandler.
 */
export function createPayGate(_config: PayGateConfig = { enforce: false }) {
  return async function payGate(
    _c: unknown,
    next: () => Promise<void>,
  ): Promise<void> {
    // v1: always pass through
    // TODO(contrib): Validate payment proof from request headers
    await next();
  };
}
