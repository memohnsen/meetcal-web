use js_sys::Date;
use leptos::prelude::*;

#[component]
pub fn TermsOfUse() -> impl IntoView {
    let current_date = Date::new_0();

    let formatted_date = format!(
        "{:02}/{:02}/{}",
        current_date.get_month() + 1,
        current_date.get_date(),
        current_date.get_full_year(),
    );

    view! {
        <main>
            <div>
                <h1>
                    "Terms of Use"
                </h1>

                <div>
                    <p>
                        "Effective Date: " {formatted_date}
                    </p>

                    <h2>
                        "1. Acceptance of Terms"
                    </h2>

                    <p>
                        "By downloading and using MeetCal, you agree to these Terms of Use. If you do not agree, please discontinue use of the app."
                    </p>

                    <h2>
                        "2. Purpose of the App"
                    </h2>

                    <p>
                        "MeetCal is designed to provide a user-friendly way to view and manage national weightlifting meet schedules. The app does not provide live scoring, athlete registration, or real-time updates from federations."
                    </p>

                    <h2>
                        "3. User Responsibilities"
                    </h2>

                    <ul>
                        <li>
                            "You agree to use MeetCal solely for its intended purpose."
                        </li>
                        <li>
                            "You acknowledge that event information may change, and you should verify details with official sources."
                        </li>
                    </ul>

                    <h2>
                        "4. Data Collection"
                    </h2>

                    <p>
                        "MeetCal collects and stores basic user information including name, email address, and role (e.g., athlete, coach, spectator). This information is used for app updates and marketing communications. For complete details about data collection and usage, please refer to our "
                        <a
                            href="/privacy-policy"
                        >
                            "Privacy Policy"
                        </a>
                        "."
                    </p>

                    <h2>
                        "5. Limitation of Liability"
                    </h2>

                    <p>
                        "We strive to provide accurate meet schedules, but we do not guarantee completeness or accuracy. We are not responsible for any scheduling conflicts, missed events, or reliance on information provided in the app."
                    </p>

                    <h2>
                        "6. Modifications to the App"
                    </h2>

                    <p>
                        "We may update or modify MeetCal without prior notice to improve functionality or add features."
                    </p>

                    <h2>
                        "7. Termination"
                    </h2>

                    <p>
                        "We reserve the right to terminate access to MeetCal at our discretion if misuse or abuse of the app occurs."
                    </p>

                    <h2>
                        "8. Contact Information"
                    </h2>

                    <p>
                        "For questions or concerns about these Terms of Use, contact us at "
                        <a
                            href="mailto:maddisen@meetcal.app"
                        >
                            "maddisen@meetcal.app"
                        </a>
                        "."
                    </p>

                    <p>
                        "By continuing to use MeetCal, you acknowledge and agree to these terms."
                    </p>
                </div>
            </div>
        </main>
    }
}
