#![cfg_attr(nightly, feature(proc_macro_span))]

use proc_macro::*;
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
#[cfg(feature = "proc_macro_span")]
use std::iter::once;

// TODO: This doesn't really work because the item is parsed before the macro
// runs, which makes the $identifiers unusable.

// #[proc_macro_attribute]
// #[proc_macro_error]
// pub fn item_for_each(attr: TokenStream, input: TokenStream) -> TokenStream {
//   let (substitution_identifier, substitution_values) =
//     parse_attr(&mut attr.into_iter());
//   // TODO: Handle there being more tokens in attr after that.
//
//   let mut output = Vec::<TokenTree>::new();
//
//   for substitution_value in substitution_values {
//     output.extend(substitute(
//       input.clone(),
//       &substitution_identifier,
//       &substitution_value,
//     ));
//   }
//
//   return TokenStream::from_iter(output);
// }

#[proc_macro]
#[proc_macro_error]
pub fn macro_wrap(input: TokenStream) -> TokenStream {
  let mut output = Vec::<TokenTree>::new();

  // println!("{:?}", input.clone().into_iter().collect::<Vec<_>>());

  let mut prior_to_previous_token = None::<TokenTree>;
  let mut previous_token = None::<TokenTree>;
  for token in input.into_iter() {
    match token {
      TokenTree::Group(ref group) => {
        // println!("{:?} {:?}", prior_to_previous_token, previous_token);
        match (prior_to_previous_token, previous_token.clone()) {
          (Some(TokenTree::Ident(ident)), Some(TokenTree::Punct(punct)))
            if ident.to_string() == "macro_for" && punct.as_char() == '!' =>
          {
            output.pop();
            output.pop();
            output.extend(macro_for(group.stream()).into_iter());
          }
          _ => {
            let mut output_group =
              Group::new(group.delimiter(), macro_wrap(group.stream()));
            output_group.set_span(group.span());
            output.push(output_group.into());
          }
        }
      }
      _ => output.push(token.clone()),
    }
    prior_to_previous_token = previous_token;
    previous_token = Some(token);
  }
  return TokenStream::from_iter(output);
}

#[proc_macro]
#[proc_macro_error]
pub fn macro_for(input: TokenStream) -> TokenStream {
  let mut tokens = input.clone().into_iter();

  let (substitution_identifier, substitution_values) = parse_attr(&mut tokens);

  let mut output = Vec::<TokenTree>::new();
  match tokens.next() {
    Some(TokenTree::Group(group)) => {
      if group.delimiter() != Delimiter::Brace {
        abort!(
          group.span_open(), "expected '{' after subsituted values";
          help = "try placing this code inside a block: `{{ {} }}`", group.to_string()
        )
      }

      for substitution_value in substitution_values {
        output.extend(substitute(
          group.stream(),
          &substitution_identifier,
          &substitution_value,
        ));
      }
    }
    Some(token) => {
      #[cfg(feature = "proc_macro_span")]
      if let Some(source) = once(token.clone())
        .chain(tokens)
        .map(|t| t.span().source_text())
        .fold(Some("".to_string()), |accum, source| {
          accum.map(|a| source.as_ref().map(|s| a + s)).flatten()
        })
      {
        abort!(
          token.span(), "expected '{' after subsituted values";
          help = "try placing this code inside a block: `{{ {} }}`", source
        )
      }
      abort!(token.span(), "expected '{' after subsituted values");
    }
    None => {
      abort_call_site!("expected '{' after subsituted values")
    }
  }

  // TODO: Handle there being more tokens after the closing brace.

  return TokenStream::from_iter(output);
}

fn parse_attr<T: Iterator<Item = TokenTree>>(
  tokens: &mut T,
) -> (Ident, Vec<Vec<TokenTree>>) {
  let substitution_identifier: Ident;
  match tokens.next() {
    Some(TokenTree::Punct(dollar)) if dollar.as_char() == '$' => {
      let token = tokens.next();
      match token {
        Some(TokenTree::Ident(ident)) => {
          #[cfg(feature = "proc_macro_span")]
          if ident.span().start().line == dollar.span().end().line
            && ident.span().start().column == dollar.span().end().column + 1
          {
            substitution_identifier = ident;
          } else {
            abort_call_site!(
              "extraneus space between '$' and substitution identifier"
            );
          }
          #[cfg(not(feature = "proc_macro_span"))]
          {
            substitution_identifier = ident;
          }
        }
        Some(token) => {
          abort!(token.span(), "missing identifier after '$'")
        }
        None => {
          abort!(dollar.span(), "missing identifier after '$'")
        }
      }
    }
    Some(TokenTree::Ident(id)) => {
      if id.to_string() == "in" {
        abort!(
          id.span(),
          "missing substitution identifier starting with '$' before 'in'"
        )
      } else {
        abort!(id.span(), "substitution identifier should start with '$'")
      }
    }
    Some(token) => {
      abort!(
        token.span(),
        "expected substitution identifier starting with '$'"
      )
    }
    None => {
      abort_call_site!("expected substitution identifier starting with '$'")
    }
  }

  match tokens.next() {
    Some(TokenTree::Ident(id)) if id.to_string() == "in" => {}
    Some(token) => {
      abort!(token.span(), "expected 'in'")
    }
    None => {
      abort_call_site!("expected 'in'")
    }
  }

  let mut substitution_values = Vec::<Vec<TokenTree>>::new();
  match tokens.next() {
    Some(TokenTree::Group(group)) => {
      if group.delimiter() != Delimiter::Bracket {
        abort!(
          group.span(),
          "expected substituted values to be enclosed in square brackets and separated with commas";
          help = "try using [] instead of {:?}", group.delimiter();
          // TODO: Emit an actual suggestion diagnostic for quick-fixes instead
          // of a help diagnostic once the feature is available. See
          // https://stackoverflow.com/q/68146335/237285#comment120447861_68146335
          // and https://github.com/rust-lang/rust/issues/54140.
          help = "like this: `[{}]`", group.stream().to_string()
        )
      }

      // Split tokens by commas.
      let mut value = Vec::<TokenTree>::new();
      for t in group.stream().into_iter() {
        match t {
          TokenTree::Punct(p) if p.as_char() == ',' => {
            substitution_values.push(value);
            value = Vec::<_>::new();
          }
          _ => value.push(t),
        }
      }
      substitution_values.push(value);
    }
    Some(token) => {
      abort!(
        token.span(),
        "expected substituted values inside square brackets and separated with commas";
        help = "like this: `[one, two, three]`"
      )
    }
    None => {
      abort_call_site!(
        "expected substituted values inside square brackets and separated with commas";
        help = "like this: `[one, two, three]`"
      )
    }
  }

  return (substitution_identifier, substitution_values);
}

fn substitute(
  tokens: TokenStream,
  substitution_id: &Ident,
  substitution_value: &Vec<TokenTree>,
) -> Vec<TokenTree> {
  let mut output = Vec::<TokenTree>::new();
  let mut previous_token = None::<TokenTree>;
  for token in tokens.into_iter() {
    match token {
      TokenTree::Ident(ref id)
        if id.to_string() == substitution_id.to_string() =>
      {
        // TODO: Require no spaces between '$' and identifier on nightly.
        match previous_token {
          Some(TokenTree::Punct(p)) if p.as_char() == '$' => {
            output.pop();
            output.extend_from_slice(&substitution_value);
          }
          _ => output.push(token.clone()),
        }
      }
      TokenTree::Group(ref group) => {
        let mut output_group = Group::new(
          group.delimiter(),
          TokenStream::from_iter(substitute(
            group.stream(),
            substitution_id,
            substitution_value,
          )),
        );
        output_group.set_span(group.span());
        output.push(output_group.into());
      }
      _ => output.push(token.clone()),
    }
    previous_token = Some(token);
  }
  return output;
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
