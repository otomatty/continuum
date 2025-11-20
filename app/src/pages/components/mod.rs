use crate::components::drawer::DrawerSide as DrawerSideComponent;
use crate::components::{
    accordion::{Accordion, AccordionContent, AccordionHeader, AccordionItem},
    alert::{Alert, AlertDescription, AlertTitle, AlertVariant},
    avatar::Avatar,
    badge::{Badge, BadgeVariant},
    breadcrumbs::{BreadcrumbItem, Breadcrumbs},
    button::{Button, ButtonVariant},
    card::{Card, CardBody, CardTitle},
    checkbox::Checkbox,
    countdown::Countdown,
    divider::Divider,
    drawer::{Drawer, DrawerContent, DrawerToggle},
    dropdown::{Dropdown, DropdownButton, DropdownItem, DropdownMenu},
    footer::Footer,
    hero::Hero,
    input::Input,
    loading::Loading,
    modal::{Modal, ModalAction, ModalBody, ModalBox, ModalFooter, ModalHeader},
    pagination::Pagination,
    popover::Popover,
    progress::{Progress, ProgressVariant},
    radio::Radio,
    range::Range,
    rating::Rating,
    select::{Select, SelectOption},
    skeleton::Skeleton,
    stats::{StatDescription, StatItem, StatTitle, StatValue, Stats},
    steps::{StepItem, StepStatus, Steps},
    table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow},
    tabs::{Tab, Tabs},
    textarea::Textarea,
    toast::{Toast, ToastVariant},
    toggle::Toggle,
    tooltip::Tooltip,
};
use chrono::Utc;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/// Renders the components showcase page.
#[component]
pub fn ComponentsPage() -> impl IntoView {
    // State for interactive demos
    let input_value = RwSignal::new("".to_string());
    let textarea_value = RwSignal::new("".to_string());
    let select_value = RwSignal::new("option1".to_string());
    let checkbox_checked = RwSignal::new(false);
    let radio_selected = RwSignal::new("option1".to_string());
    let radio_option1_checked = RwSignal::new(true);
    let radio_option2_checked = RwSignal::new(false);
    let toggle_checked = RwSignal::new(false);
    let range_value = RwSignal::new(50.0);
    let pagination_page = RwSignal::new(1);
    let modal_open = RwSignal::new(false);
    let dropdown_open = RwSignal::new(false);
    let drawer_open = RwSignal::new(false);
    let popover_open = RwSignal::new(false);
    let accordion_open = RwSignal::new(false);
    let rating_value = RwSignal::new(3.0);

    view! {
        <div class="space-y-12">
            <div class="text-center py-8">
                <h1 class="text-5xl font-bold mb-4">"Component Showcase"</h1>
                <p class="text-xl text-gray-600">
                    "Browse and test all available UI components"
                </p>
            </div>

            // Form Components
            <section class="space-y-6">
                <h2 class="text-3xl font-bold mb-4">"Form Components"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <Card>
                        <CardTitle>"Input"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Text input field"</p>
                            <Input
                                placeholder="Enter text..."
                                value=input_value.read_only()
                                on_input=Callback::new(move |ev: leptos::ev::InputEvent| {
                                    if let Some(target) = ev.target() {
                                        if let Ok(input) = target.dyn_into::<leptos::web_sys::HtmlInputElement>() {
                                            input_value.set(input.value());
                                        }
                                    }
                                })
                            />
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Textarea"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Multi-line text input"</p>
                            <Textarea
                                placeholder="Enter text..."
                                rows=3
                                value=textarea_value.read_only()
                                on_input=Callback::new(move |ev: leptos::ev::InputEvent| {
                                    if let Some(target) = ev.target() {
                                        if let Ok(textarea) = target.dyn_into::<leptos::web_sys::HtmlTextAreaElement>() {
                                            textarea_value.set(textarea.value());
                                        }
                                    }
                                })
                            />
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Select"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Dropdown selection"</p>
                            <Select
                                value=select_value.read_only()
                                on_change=Callback::new(move |ev: leptos::web_sys::Event| {
                                    if let Some(target) = ev.target() {
                                        if let Ok(select) = target.dyn_into::<leptos::web_sys::HtmlSelectElement>() {
                                            select_value.set(select.value());
                                        }
                                    }
                                })
                            >
                                <SelectOption value="option1".to_string()>"Option 1"</SelectOption>
                                <SelectOption value="option2".to_string()>"Option 2"</SelectOption>
                                <SelectOption value="option3".to_string()>"Option 3"</SelectOption>
                            </Select>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Checkbox"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Checkbox input"</p>
                            <div class="flex items-center gap-2">
                                <Checkbox
                                    checked=checkbox_checked.read_only()
                                    on_change=Callback::new(move |ev: leptos::web_sys::Event| {
                                        if let Some(target) = ev.target() {
                                            if let Ok(checkbox) = target.dyn_into::<leptos::web_sys::HtmlInputElement>() {
                                                checkbox_checked.set(checkbox.checked());
                                            }
                                        }
                                    })
                                />
                                <label>"Check me"</label>
                            </div>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Radio"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Radio button group"</p>
                            <div class="flex flex-col gap-2">
                                <div class="flex items-center gap-2">
                                    <Radio
                                        name="demo-radio".to_string()
                                        value="option1".to_string()
                                        checked=radio_option1_checked.read_only()
                                        on_change=Callback::new(move |_| {
                                            radio_option1_checked.set(true);
                                            radio_option2_checked.set(false);
                                            radio_selected.set("option1".to_string());
                                        })
                                    />
                                    <label>"Option 1"</label>
                                </div>
                                <div class="flex items-center gap-2">
                                    <Radio
                                        name="demo-radio".to_string()
                                        value="option2".to_string()
                                        checked=radio_option2_checked.read_only()
                                        on_change=Callback::new(move |_| {
                                            radio_option1_checked.set(false);
                                            radio_option2_checked.set(true);
                                            radio_selected.set("option2".to_string());
                                        })
                                    />
                                    <label>"Option 2"</label>
                                </div>
                            </div>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Toggle"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Toggle switch"</p>
                            <Toggle
                                checked=toggle_checked.read_only()
                                on_change=Callback::new(move |ev: leptos::web_sys::Event| {
                                    if let Some(target) = ev.target() {
                                        if let Ok(toggle) = target.dyn_into::<leptos::web_sys::HtmlInputElement>() {
                                            toggle_checked.set(toggle.checked());
                                        }
                                    }
                                })
                            />
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Range"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Range slider"</p>
                            <Range
                                min=0.0
                                max=100.0
                                step=1.0
                                value=range_value.read_only()
                                on_input=Callback::new(move |ev: leptos::ev::InputEvent| {
                                    if let Some(target) = ev.target() {
                                        if let Ok(range) = target.dyn_into::<leptos::web_sys::HtmlInputElement>() {
                                            if let Ok(val) = range.value().parse::<f64>() {
                                                range_value.set(val);
                                            }
                                        }
                                    }
                                })
                            />
                            <p class="text-sm mt-2">"Value: " {move || range_value.get()}</p>
                        </CardBody>
                    </Card>
                </div>
            </section>

            // Button Components
            <section class="space-y-6">
                <h2 class="text-3xl font-bold mb-4">"Button Components"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <Card>
                        <CardTitle>"Button"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Button with variants"</p>
                            <div class="flex flex-col gap-2">
                                <Button variant=ButtonVariant::Primary>"Primary"</Button>
                                <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                                <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                            </div>
                        </CardBody>
                    </Card>
                </div>
            </section>

            // Display Components
            <section class="space-y-6">
                <h2 class="text-3xl font-bold mb-4">"Display Components"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <Card>
                        <CardTitle>"Card"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Card container"</p>
                            <Card>
                                <CardBody>
                                    <p>"Card content"</p>
                                </CardBody>
                            </Card>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Badge"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Badge labels"</p>
                            <div class="flex flex-wrap gap-2">
                                <Badge variant=BadgeVariant::Primary>"Primary"</Badge>
                                <Badge variant=BadgeVariant::Success>"Success"</Badge>
                                <Badge variant=BadgeVariant::Warning>"Warning"</Badge>
                                <Badge variant=BadgeVariant::Error>"Error"</Badge>
                            </div>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Avatar"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"User avatar"</p>
                            <Avatar
                                src="https://api.dicebear.com/7.x/avataaars/svg?seed=John".to_string()
                                alt="Avatar".to_string()
                            />
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Alert"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Alert messages"</p>
                            <div class="space-y-2">
                                <Alert variant=AlertVariant::Info>
                                    <AlertTitle>"Info"</AlertTitle>
                                    <AlertDescription>"This is an info alert"</AlertDescription>
                                </Alert>
                                <Alert variant=AlertVariant::Success>
                                    <AlertTitle>"Success"</AlertTitle>
                                    <AlertDescription>"This is a success alert"</AlertDescription>
                                </Alert>
                            </div>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Progress"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Progress bar"</p>
                            <Progress value=75 max=100 />
                            <Progress value=50 max=100 variant=ProgressVariant::Success />
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Loading"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Loading spinner"</p>
                            <Loading />
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Skeleton"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Loading skeleton"</p>
                            <Skeleton />
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Divider"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Divider line"</p>
                            <p>"Content above"</p>
                            <Divider />
                            <p>"Content below"</p>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Stats"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Statistics display"</p>
                            <Stats>
                                <StatItem>
                                    <StatTitle>"Total Users"</StatTitle>
                                    <StatValue>"1,234"</StatValue>
                                    <StatDescription>"21% more than last month"</StatDescription>
                                </StatItem>
                            </Stats>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Countdown"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Countdown timer"</p>
                            <Countdown target_date=Utc::now().timestamp() + 3600 />
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Rating"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Star rating"</p>
                            <Rating
                                value=rating_value.get()
                                on_change=Callback::new(move |val| rating_value.set(val))
                            />
                        </CardBody>
                    </Card>
                </div>
            </section>

            // Navigation Components
            <section class="space-y-6">
                <h2 class="text-3xl font-bold mb-4">"Navigation Components"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <Card>
                        <CardTitle>"Breadcrumbs"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Breadcrumb navigation"</p>
                            <Breadcrumbs>
                                <BreadcrumbItem href="/">"Home"</BreadcrumbItem>
                                <BreadcrumbItem href="/components">"Components"</BreadcrumbItem>
                                <BreadcrumbItem>"Current"</BreadcrumbItem>
                            </Breadcrumbs>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Pagination"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Page navigation"</p>
                            <Pagination
                                current_page=pagination_page.get()
                                total_pages=5
                                on_page_change=Callback::new(move |page| {
                                    pagination_page.set(page);
                                })
                            />
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Tabs"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Tab navigation"</p>
                            <Tabs>
                                <Tab index=0>"Tab 1"</Tab>
                                <Tab index=1>"Tab 2"</Tab>
                                <Tab index=2>"Tab 3"</Tab>
                            </Tabs>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Steps"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Step indicator"</p>
                            <Steps>
                                <StepItem status=StepStatus::Primary>"Step 1"</StepItem>
                                <StepItem status=StepStatus::Default>"Step 2"</StepItem>
                                <StepItem status=StepStatus::Default>"Step 3"</StepItem>
                            </Steps>
                        </CardBody>
                    </Card>
                </div>
            </section>

            // Layout Components
            <section class="space-y-6">
                <h2 class="text-3xl font-bold mb-4">"Layout Components"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <Card>
                        <CardTitle>"Hero"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Hero section"</p>
                            <Hero>
                                <div class="hero-content">
                                    <h1 class="text-2xl font-bold">"Hero Content"</h1>
                                </div>
                            </Hero>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Accordion"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Collapsible content"</p>
                            <Accordion>
                                <AccordionItem open=accordion_open.read_only() set_open=accordion_open.write_only()>
                                    <AccordionHeader>"Item 1"</AccordionHeader>
                                    <AccordionContent>"Content 1"</AccordionContent>
                                </AccordionItem>
                                <AccordionItem>
                                    <AccordionHeader>"Item 2"</AccordionHeader>
                                    <AccordionContent>"Content 2"</AccordionContent>
                                </AccordionItem>
                            </Accordion>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Footer"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Page footer"</p>
                            <Footer>
                                <p>"Footer content"</p>
                            </Footer>
                        </CardBody>
                    </Card>
                </div>
            </section>

            // Overlay Components
            <section class="space-y-6">
                <h2 class="text-3xl font-bold mb-4">"Overlay Components"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <Card>
                        <CardTitle>"Modal"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Modal dialog"</p>
                            <Button on_click=Callback::new(move |_| modal_open.set(true))>"Open Modal"</Button>
                            <Modal open=modal_open.read_only() on_close=Callback::new(move |_| modal_open.set(false))>
                                <ModalBox>
                                    <ModalHeader>"Modal Title"</ModalHeader>
                                    <ModalBody>"Modal content goes here"</ModalBody>
                                    <ModalFooter>
                                        <ModalAction on_click=Callback::new(move |_| modal_open.set(false))>"Close"</ModalAction>
                                    </ModalFooter>
                                </ModalBox>
                            </Modal>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Dropdown"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Dropdown menu"</p>
                            <Dropdown
                                open=dropdown_open.read_only()
                                on_toggle=Callback::new(move |_| dropdown_open.set(!dropdown_open.get()))
                            >
                                <DropdownButton>"Dropdown"</DropdownButton>
                                <DropdownMenu>
                                    <DropdownItem>"Item 1"</DropdownItem>
                                    <DropdownItem>"Item 2"</DropdownItem>
                                    <DropdownItem>"Item 3"</DropdownItem>
                                </DropdownMenu>
                            </Dropdown>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Drawer"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Side drawer"</p>
                            <Button on_click=Callback::new(move |_| drawer_open.set(true))>"Open Drawer"</Button>
                            <Drawer
                                open=drawer_open.read_only()
                                on_close=Callback::new(move |_| drawer_open.set(false))
                            >
                                <DrawerContent>
                                    <DrawerToggle>"Close"</DrawerToggle>
                                </DrawerContent>
                                <DrawerSideComponent>
                                    <p>"Drawer content"</p>
                                </DrawerSideComponent>
                            </Drawer>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Tooltip"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Tooltip"</p>
                            <Tooltip content="This is a tooltip".to_string()>
                                <Button>"Hover me"</Button>
                            </Tooltip>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Popover"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Popover"</p>
                            <Popover
                                open=popover_open.read_only()
                                content="Popover content".to_string()
                                on_toggle=Callback::new(move |_| popover_open.set(!popover_open.get()))
                            >
                                <Button>"Toggle Popover"</Button>
                            </Popover>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardTitle>"Toast"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Toast notification"</p>
                            <div class="space-y-2">
                                <Toast message="Info toast".to_string() variant=ToastVariant::Info />
                                <Toast message="Success toast".to_string() variant=ToastVariant::Success />
                            </div>
                        </CardBody>
                    </Card>
                </div>
            </section>

            // Data Display
            <section class="space-y-6">
                <h2 class="text-3xl font-bold mb-4">"Data Display"</h2>
                <div class="grid grid-cols-1 gap-6">
                    <Card>
                        <CardTitle>"Table"</CardTitle>
                        <CardBody>
                            <p class="text-sm text-gray-600 mb-4">"Data table"</p>
                            <Table>
                                <TableHead>
                                    <TableRow>
                                        <TableHeader>"Name"</TableHeader>
                                        <TableHeader>"Status"</TableHeader>
                                        <TableHeader>"Role"</TableHeader>
                                    </TableRow>
                                </TableHead>
                                <TableBody>
                                    <TableRow>
                                        <TableCell>"John Doe"</TableCell>
                                        <TableCell>"Active"</TableCell>
                                        <TableCell>"Admin"</TableCell>
                                    </TableRow>
                                    <TableRow>
                                        <TableCell>"Jane Smith"</TableCell>
                                        <TableCell>"Active"</TableCell>
                                        <TableCell>"User"</TableCell>
                                    </TableRow>
                                </TableBody>
                            </Table>
                        </CardBody>
                    </Card>
                </div>
            </section>
        </div>
    }
}
