#![feature(const_fn_floating_point_arithmetic)]

mod units;
mod generator;

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
                <label for="front">"Separate front plate?"</label>
                <input type="checkbox" id="front" value={value.get_untracked().front}/>
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
                    <div class="container">
                    <svg xmlns="http://www.w3.org/2000/svg" class="ge-export-svg-dark" filter="invert(100%) hue-rotate(180deg)" viewBox="-0.5 -0.5 762 402"><defs/><path fill="none" stroke="#000" stroke-dasharray="3 3" stroke-miterlimit="10" d="M360 340V60M600 340V60M360 60h240M360 340h240M320 340V60M640 340V60" pointer-events="stroke"/><path fill="none" stroke="#000" stroke-miterlimit="10" d="M600 60h40M360 60h-40M360 340h-40M640 340h-40M660 340V60M640 60h20M640 340h20M300 340V60M320 60h-20M320 340h-20M360 0h240" pointer-events="stroke"/><path fill="none" stroke="#000" stroke-dasharray="3 3" stroke-miterlimit="10" d="M360 20h240M360 380h240" pointer-events="stroke"/><path fill="none" stroke="#000" stroke-miterlimit="10" d="M360 400h240M600 20v40M600 0v20M360 0v20M360 20v40M600 340v40M600 380v20M360 340v40M360 380v20M720 280v-60M760 280v-60M720 220h40M720 280h40M720 200v-60M760 200v-60M720 140h40M720 200h40M0 340V60M0 60h240M240 340V60M0 340h240" pointer-events="stroke"/><path fill="none" stroke="#000" stroke-dasharray="3 3" stroke-miterlimit="10" d="M720 180h40M720 260h40" pointer-events="stroke"/></svg>
                    </div>
                </div>
                <div class="vertical">
                    <h2>Render</h2>
                    <div class="container full">
                        <canvas class="full"></canvas>
                    </div>
                </div>
            </div>
        }
    })
}
