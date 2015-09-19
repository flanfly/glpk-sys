extern crate glpk_sys as glpk;
extern crate libc;

use std::ffi::CString;
use std::ptr;
use libc::{c_int,c_double};

fn main() {
    unsafe {
        let lp = glpk::glp_create_prob();

        glpk::glp_set_prob_name(lp, CString::new("sample").unwrap().as_ptr());
        glpk::glp_set_obj_dir(lp, 2);
        glpk::glp_add_rows(lp, 3);

        glpk::glp_set_row_name(lp, 1, CString::new("p").unwrap().as_ptr());
        glpk::glp_set_row_bnds(lp, 1, 3, 0.0, 100.0);
        glpk::glp_set_row_name(lp, 2, CString::new("q").unwrap().as_ptr());
        glpk::glp_set_row_bnds(lp, 2, 3, 0.0, 600.0);
        glpk::glp_set_row_name(lp, 3, CString::new("r").unwrap().as_ptr());
        glpk::glp_set_row_bnds(lp, 3, 3, 0.0, 300.0);

        glpk::glp_add_cols(lp, 3);
        glpk::glp_set_col_name(lp, 1, CString::new("x1").unwrap().as_ptr());
        glpk::glp_set_col_bnds(lp, 1, 2, 0.0, 0.0);
        glpk::glp_set_obj_coef(lp, 1, 10.0);
        glpk::glp_set_col_name(lp, 2, CString::new("x2").unwrap().as_ptr());
        glpk::glp_set_col_bnds(lp, 2, 2, 0.0, 0.0);
        glpk::glp_set_obj_coef(lp, 2, 6.0);
        glpk::glp_set_col_name(lp, 3, CString::new("x3").unwrap().as_ptr());
        glpk::glp_set_col_bnds(lp, 3, 2, 0.0, 0.0);
        glpk::glp_set_obj_coef(lp, 3, 4.0);

        let mut ia: [c_int; 1001usize] = [0; 1001];
        let mut ja: [c_int; 1001usize] = [0; 1001];
        let mut ar: [c_double; 1001usize] = [0.0; 1001];

        ia[1] = 1; ja[1] = 1; ar[1] =  1.0; /* a[1,1] =  1 */
        ia[2] = 1; ja[2] = 2; ar[2] =  1.0; /* a[1,2] =  1 */
        ia[3] = 1; ja[3] = 3; ar[3] =  1.0; /* a[1,3] =  1 */
        ia[4] = 2; ja[4] = 1; ar[4] = 10.0; /* a[2,1] = 10 */
        ia[5] = 3; ja[5] = 1; ar[5] =  2.0; /* a[3,1] =  2 */
        ia[6] = 2; ja[6] = 2; ar[6] =  4.0; /* a[2,2] =  4 */
        ia[7] = 3; ja[7] = 2; ar[7] =  2.0; /* a[3,2] =  2 */
        ia[8] = 2; ja[8] = 3; ar[8] =  5.0; /* a[2,3] =  5 */
        ia[9] = 3; ja[9] = 3; ar[9] =  6.0; /* a[3,3] =  6 */


        glpk::glp_load_matrix(lp, 9, ia.as_ptr(), ja.as_ptr(), ar.as_ptr());

        glpk::glp_simplex(lp, ptr::null());
        let z = glpk::glp_get_obj_val(lp);
        let x1 = glpk::glp_get_col_prim(lp, 1);
        let x2 = glpk::glp_get_col_prim(lp, 2);
        let x3 = glpk::glp_get_col_prim(lp, 3);

        println!("z = {}; x1 = {}; x2 = {}; x3 = {}", z, x1, x2, x3);
        glpk::glp_delete_prob(lp);
    }
}
