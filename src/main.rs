#![feature(const_fn_floating_point_arithmetic)]

mod generator;
mod units;

use leptos::*;
use units::{CaseDesign, CaseMaterial};

#[component]
pub fn CaseConfigurator(
    /// The starting value for the component
    initial_value: units::CaseDesign,
) -> impl IntoView {
    let (value, set_value) = create_signal(initial_value);

    view! {
        <div class="container">
            <div class="input">
                <label for="material">"Material:"</label>

                <input type="radio" id="sheet_metal" name="material" value={value.get_untracked().material == CaseMaterial::SheetMetal}/>
                <label for="sheet_metal">"Sheet metal"</label>

                <input type="radio" id="wood" name="material" value={value.get_untracked().material != CaseMaterial::SheetMetal}/>
                <label for="wood">Wood</label>
            </div>

            <div class="input">
                <label for="thickness">"Material thickness/tolerance (mm)"</label>
                <input type="number" id="thickness" min="1" max="5" value={u8::from(value.get_untracked().thickness)}/>
            </div>

            <div class="input">
                <label for="ears">"Include ears?"</label>
                <input type="checkbox" id="ears" value={value.get_untracked().ears}/>
            </div>

            <div class="input">
                <label for="height">"Case height (rack units)"</label>
                <select name="height" id="height">
                    <option value="1">"1U"</option>
                    <option value="2">"2U"</option>
                    <option value="3">"3U"</option>
                    <option value="4">"4U"</option>
                    <option value="5">"5U"</option>
                    <option value="6">"6U"</option>
                </select>
            </div>

            <div class="input">
                <label for="width">"Case width"</label>
                <select name="width" id="width">
                    <option value="1">"Full"</option>
                    <option value="2">"Half"</option>
                </select>
            </div>

            <div class="input">
                <label for="depth">"Depth (mm)"</label>
                <input type="number" id="depth" min="100" max="10000" value={u16::from(value.get_untracked().depth)}/>
            </div>

            <button class="full" on:click=move |_| set_value.set(CaseDesign::default())>"Reset"</button>
        </div>
    }
}

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <h1>Rack Case Generator</h1>
            <div class="horizontal">
                <div class="vertical limited">
                    <h2>Configurator</h2>
                    <CaseConfigurator
                        initial_value=CaseDesign::default()
                    />
                    <h2>Output</h2>
                    <div class="container" 
                    inner_html={generator::generate(&CaseDesign::default()).to_string()}
                    >
                    //{generator::generate(&CaseDesign::default()).to_string()}
                    </div>
                </div>
                <div class="vertical">
                    <h2>Render</h2>
                    <div class="container full">
                        <span>tbd</span>
                        <canvas class="full"></canvas>
                    </div>
                </div>
            </div>
        }
    })
}
