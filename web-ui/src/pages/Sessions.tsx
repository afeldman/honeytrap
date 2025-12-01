/**
 * @fileoverview Honeypot sessions viewer page.
 * Displays detailed information about honeypot sessions.
 *
 * @example
 * // Route: /sessions
 * // Will show SSH, HTTP, MySQL sessions with details
 */

/**
 * Sessions page component.
 *
 * @returns {JSX.Element} Sessions viewer page
 */
export default function Sessions() {
    return (
        <div>
            <h1 className="text-3xl font-bold text-white mb-6">
                Honeypot Sessions
            </h1>
            <div className="bg-gray-800 rounded-lg p-6">
                <p className="text-gray-400">Session details coming soon...</p>
            </div>
        </div>
    );
}
