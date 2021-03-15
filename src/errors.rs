/*
 *     This file is part of rad.
 *
 *   rad is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.
 *
 *   rad is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 *
 *   You should have received a copy of the GNU General Public License
 *
 *  along with rad.  If not, see <https://www.gnu.org/licenses/>.
 */
pub struct RadError(String);

impl std::fmt::Debug for RadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn error<T>(msg: T) -> RadError
where
    T: ToString,
{
    RadError(msg.to_string())
}

impl From<std::io::Error> for RadError {
    fn from(e: std::io::Error) -> Self {
        Self(format!("{}", e))
    }
}

pub type Result<T> = std::result::Result<T, RadError>;
