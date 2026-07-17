use crate::components::{footer::Footer, header::Header};
use js_sys::Date;
use leptos::prelude::*;

#[component]
pub fn PrivacyPage() -> impl IntoView {
    view! {
        <Header />
        <PrivacyPolicy />
        <Footer />
    }
}

#[component]
pub fn PrivacyPolicy() -> impl IntoView {
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
                    "Privacy Policy"
                </h1>

                <div>
                    <p>
                        "Effective Date: " {formatted_date}
                    </p>

                    <h2>
                        "1. Introduction"
                    </h2>

                    <p>
                        "Welcome to MeetCal! Your privacy is important to us. This Privacy Policy outlines how we handle your data when you use our app."
                    </p>

                    <h2>
                        "2. Data Collection & Usage"
                    </h2>

                    <p>
                        "We collect and store the following basic information from users who choose to share it:"
                    </p>

                    <ul>
                        <li>"Name"</li>
                        <li>"Email address"</li>
                    </ul>

                    <p>
                        "This information is used solely for:"
                    </p>

                    <ul>
                        <li>
                            "Sending updates about app changes and new features"
                        </li>
                        <li>
                            "Marketing communications about relevant weightlifting events and services"
                        </li>
                        <li>
                            "Tying purchases to the user account for cross-device access"
                        </li>
                    </ul>

                    <p>
                        "We will never sell or share your personal information with third parties."
                    </p>

                    <h2>
                        "3. Permissions & Third-Party Services"
                    </h2>

                    <ul>
                        <li>
                            "Calendar Access: If you choose to save events to your device's calendar, MeetCal will request permission to access your calendar. This data is not stored or transmitted by MeetCal."
                        </li>
                        <li>
                            "Internet Access: The app requires internet access to ensure up-to-date meet schedules and to sync user preferences."
                        </li>
                    </ul>

                    <h2>
                        "4. Security"
                    </h2>

                    <p>
                        "We implement appropriate security measures to protect your personal information. Your data is stored securely and accessed only by authorized personnel for the purposes stated above."
                    </p>

                    <h2>
                        "5. Your Rights"
                    </h2>

                    <p>
                        "You have the right to:"
                    </p>

                    <ul>
                        <li>"Access your personal data"</li>
                        <li>"Correct your personal data"</li>
                        <li>"Delete your personal data"</li>
                        <li>"Opt out of marketing communications"</li>
                    </ul>

                    <h2>
                        "6. Changes to This Policy"
                    </h2>

                    <p>
                        "If any updates are made to this policy, they will be reflected in a new version within the app, and we will notify users via email of any significant changes."
                    </p>

                    <h2>
                        "7. Contact Us"
                    </h2>

                    <p>
                        "If you have any questions about this Privacy Policy or would like to exercise your data rights, please contact us at "
                        <a
                            href="mailto:maddisen@meetcal.app"
                        >
                            "maddisen@meetcal.app"
                        </a>
                        "."
                    </p>
                </div>
            </div>
        </main>
    }
}
