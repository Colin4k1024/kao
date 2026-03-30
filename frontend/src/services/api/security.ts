import axios from 'axios';
import type { AxiosResponse } from 'axios';

// Base URL for security monitoring API
const BASE_URL = '/api/monitoring/security';

// Security scan check result
export interface SecurityCheck {
  name: string;
  status: 'pass' | 'fail' | 'warning';
  details: string;
}

// Security scan summary
export interface SecurityScanSummary {
  total_checks: number;
  passed_checks: number;
  failed_checks: number;
  warning_checks: number;
}

// Overall security scan result
export interface SecurityScanResult {
  status: 'healthy' | 'warning' | 'critical';
  checks: SecurityCheck[];
  summary: SecurityScanSummary;
}

// Password health information for a user
export interface PasswordHealth {
  user_id: string;
  username: string;
  status: 'valid' | 'expiring_soon' | 'expired' | 'force_change';
  days_remaining: number | null;
  expires_at: string | null;
}

// Locked account information
export interface LockedAccount {
  user_id: string;
  username: string;
  locked_until: string;
  reason: string | null;
}

// Failed login attempt record
export interface FailedLoginAttempt {
  user_id: string | null;
  username: string;
  ip_address: string;
  attempt_count: number;
  last_attempt: string;
}

// Suspicious input pattern from audit log
export interface SuspiciousInput {
  id: string;
  username: string | null;
  ip_address: string;
  event_type: string;
  details: Record<string, unknown>;
  created_at: string;
}

// Permission denied event
export interface PermissionDeniedEvent {
  id: string;
  user_id: string | null;
  username: string | null;
  ip_address: string;
  event_type: string;
  details: Record<string, unknown>;
  created_at: string;
}

// Security event summary
export interface SecurityEventSummary {
  total_events: number;
  permission_denied_count: number;
  suspicious_input_count: number;
  brute_force_attempts: number;
}

// Brute force detection result
export interface BruteForceDetection {
  ip_address: string;
  attempt_count: number;
  is_blocked: boolean;
  blocked_until: string | null;
}

// Security events response
export interface SecurityEventsResponse {
  summary: SecurityEventSummary;
  locked_accounts: LockedAccount[];
  recent_failed_attempts: FailedLoginAttempt[];
  brute_force_detection: BruteForceDetection[];
  suspicious_inputs: SuspiciousInput[];
  permission_denied_events: PermissionDeniedEvent[];
}

// Security scan type
export type SecurityScanType = 'configuration' | 'input-validation' | 'authentication' | 'authorization';

/**
 * Fetch full security scan
 * GET /api/monitoring/security/scan
 */
export async function fetchSecurityScan(): Promise<SecurityScanResult> {
  try {
    const response: AxiosResponse<SecurityScanResult> = await axios.get(`${BASE_URL}/scan`);
    return response.data;
  } catch (error) {
    console.error('Failed to fetch security scan:', error);
    throw error;
  }
}

/**
 * Fetch security scan by type
 * GET /api/monitoring/security/scan/${type}
 * @param type - Scan type: configuration, input-validation, authentication, authorization
 */
export async function fetchSecurityScanByType(type: SecurityScanType): Promise<SecurityScanResult> {
  try {
    const response: AxiosResponse<SecurityScanResult> = await axios.get(`${BASE_URL}/scan/${type}`);
    return response.data;
  } catch (error) {
    console.error(`Failed to fetch ${type} security scan:`, error);
    throw error;
  }
}

/**
 * Fetch security events
 * GET /api/monitoring/security/events
 */
export async function fetchSecurityEvents(): Promise<SecurityEventsResponse> {
  try {
    const response: AxiosResponse<SecurityEventsResponse> = await axios.get(`${BASE_URL}/events`);
    return response.data;
  } catch (error) {
    console.error('Failed to fetch security events:', error);
    throw error;
  }
}

/**
 * Fetch password health for a user
 * GET /api/monitoring/security/password-health/:user_id
 * @param userId - User ID
 */
export async function fetchPasswordHealth(userId: string): Promise<PasswordHealth> {
  try {
    const response: AxiosResponse<PasswordHealth> = await axios.get(`${BASE_URL}/password-health/${userId}`);
    return response.data;
  } catch (error) {
    console.error(`Failed to fetch password health for user ${userId}:`, error);
    throw error;
  }
}
