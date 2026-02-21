//  <table>
//   <tr>
//     <th>Company</th>
//     <th>Contact</th>
//     <th>Country</th>
//   </tr>
//   <tr>
//     <td>Alfreds Futterkiste</td>
//     <td>Maria Anders</td>
//     <td>Germany</td>
//   </tr>
//   <tr>
//     <td>Centro comercial Moctezuma</td>
//     <td>Francisco Chang</td>
//     <td>Mexico</td>
//   </tr>
// </table> 
use maud::{Render,html,Markup};

pub struct Table {
    pub headers: Vec<String>,
    pub data: Vec<Vec<String>>
}

impl Render for Table {
    fn render(&self) -> Markup {        
        html! {

            @let headers = &self.headers;
            @let data = &self.data;

            @if headers.len() == data.len() {
                table {
                    tr {
                        @for col in headers {
                            th {(col)}
                        }
                    }

                    @for row in data {
                        tr {
                            @for col in row {
                                th {(col)}
                            }
                        }
                    }
                }
            }

        }
    }
}
