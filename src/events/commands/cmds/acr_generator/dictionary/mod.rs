mod a_list;
mod b_list;
mod c_list;
mod d_list;
mod e_list;
mod f_list;
mod g_list;
mod h_list;
mod i_list;
mod j_list;
mod k_list;
mod l_list;
mod m_list;
mod n_list;
mod o_list;
mod p_list;
mod q_list;
mod r_list;
mod s_list;
mod t_list;
mod u_list;
mod v_list;
mod w_list;
mod x_list;
mod y_list;
mod z_list;

use a_list::A_LIST;
use b_list::B_LIST;
use c_list::C_LIST;
use d_list::D_LIST;
use e_list::E_LIST;
use f_list::F_LIST;
use g_list::G_LIST;
use h_list::H_LIST;
use i_list::I_LIST;
use j_list::J_LIST;
use k_list::K_LIST;
use l_list::L_LIST;
use m_list::M_LIST;
use n_list::N_LIST;
use o_list::O_LIST;
use p_list::P_LIST;
use q_list::Q_LIST;
use r_list::R_LIST;
use s_list::S_LIST;
use t_list::T_LIST;
use u_list::U_LIST;
use v_list::V_LIST;
use w_list::W_LIST;
use x_list::X_LIST;
use y_list::Y_LIST;
use z_list::Z_LIST;

const DICT: [&[&str]; 26] = [
    A_LIST, B_LIST, C_LIST, D_LIST, E_LIST, F_LIST, G_LIST, H_LIST, I_LIST, J_LIST, K_LIST, L_LIST,
    M_LIST, N_LIST, O_LIST, P_LIST, Q_LIST, R_LIST, S_LIST, T_LIST, U_LIST, V_LIST, W_LIST, X_LIST,
    Y_LIST, Z_LIST,
];

pub fn get_elem_list(character: char) -> Option<&'static [&'static str]> {
    const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    match ALPHABET.find(character.to_ascii_uppercase()) {
        Some(index) => Some(DICT[index]),
        _ => None,
    }
}
