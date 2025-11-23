use crate::components::{fade_in::FadeIn, heading::SectionTitle, section::Section};
use leptos::prelude::*;

/**
 * AboutContinuum Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/home/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ leptos::prelude
 *   ├─ app/src/components/section/mod.rs
 *   ├─ app/src/components/fade_in/mod.rs
 *   └─ app/src/components/heading/mod.rs
 */
#[component]
pub fn AboutContinuum() -> impl IntoView {
    view! {
        <Section>
            <FadeIn>
                <SectionTitle title="Continuumとは".to_string() margin_bottom="mb-12" />
            </FadeIn>

            <FadeIn delay="200".to_string()>
                <div class="max-w-3xl mx-auto space-y-6">
                    <p class="text-gray-600">
                        "Continuum（コンティニュアム）は、ラテン語で「連続体」「連続性」を意味する言葉です。"
                    </p>
                    <p class="text-gray-600 leading-relaxed">
                        "このアプリケーションは、エンジニアの成長を組織全体で支援することを目的としています。
                        OSS活動を通じた実践的な学習と知見共有を促進し、エンジニアの成長が途切れることなく
                        継続的に発展していく様子を「Continuum（連続体）」という言葉で表現しました。
                        個人の成長が組織全体の成長につながり、それがまた個人の成長を後押しするという
                        好循環を生み出すプラットフォームとして、この名前を選びました。"
                    </p>
                </div>
            </FadeIn>
        </Section>
    }
}
