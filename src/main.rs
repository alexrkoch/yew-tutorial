mod video_data;
use video_data::get_videos;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let videos = get_videos();

    let videos_html = videos
        .iter()
        .map(|video| {
            html! {
                <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect::<Vec<_>>();

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                { for videos_html }
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
