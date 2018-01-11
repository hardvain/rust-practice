struct Sample {
  key: i32,
}
impl Sample {
  pub fn owned(self) {}
  pub fn shared_ref(&self) {}
  pub fn mutable_ref(&self) {}
}
fn taking_owned_sample(sample: Sample) {}
fn taking_shared_sample(sample: &Sample) {}
fn taking_mutable_sample(sample: &mut Sample) {}

fn main() {
  immutable_binding_method_calls();
  mutable_binding_method_calls();
  immutable_binding_function_calls();
  mutable_binding_method_calls();
}
fn immutable_binding_function_calls() {
  {
    let sample = Sample { key: 1 };
    taking_owned_sample(sample);
    // cannot use below calls because value has been moved
    // taking_owned_sample(sample);
    // taking_shared_sample(sample);
    // taking_mutable_sample(sample);
  }
  {
    let sample = Sample { key: 1 };
    taking_shared_sample(&sample);
    taking_shared_sample(&sample);
    // taking_mutable_sample(&mut sample);
    // taking_mutable_sample(&mut sample);
    taking_owned_sample(sample);
  }
  {
    let sample = Sample { key: 1 };
    // Not at all possible
    // taking_mutable_sample(&mut sample)
  }
}
fn mutable_binding_function_calls() {
  {
    let mut sample = Sample { key: 1 };
    taking_owned_sample(sample);
    // cannot use below calls because value has been moved
    // taking_owned_sample(sample);
    // taking_shared_sample(sample);
    // taking_mutable_sample(sample);
  }
  {
    let mut sample = Sample { key: 1 };
    taking_shared_sample(&sample);
    taking_shared_sample(&sample);
    taking_mutable_sample(&mut sample);
    taking_mutable_sample(&mut sample);
    taking_owned_sample(sample);
  }
  {
    let mut sample = Sample { key: 1 };
    taking_shared_sample(&sample);
    taking_shared_sample(&sample);
    taking_owned_sample(sample);
    // taking_mutable_sample(&mut sample);
    // taking_mutable_sample(&mut sample);
  }
  {
    let mut sample = Sample { key: 1 };
    // Not at all possible
    taking_mutable_sample(&mut sample);
    taking_shared_sample(&sample);
    taking_shared_sample(&sample);
    taking_owned_sample(sample);
  }
}
fn immutable_binding_method_calls() {
  {
    let sample = Sample { key: 1 };
    sample.owned();
    // cannot use below calls because value has been moved
    // sample.owned();
    // sample.shared_ref();
    // sample.mutable_ref();
  }
  {
    let sample = Sample { key: 1 };
    sample.shared_ref();
    sample.shared_ref();
    sample.mutable_ref();
    sample.mutable_ref();
    sample.owned();
    // sample.owned();
  }
  {
    let sample = Sample { key: 1 };
    sample.shared_ref();
    sample.owned();
    // cannot call below because value has been moved
    // sample.mutable_ref();
  }
  {
    let sample = Sample { key: 1 };
    sample.mutable_ref();
    sample.mutable_ref();
    sample.shared_ref();
    sample.shared_ref();
    sample.owned();
  }
}

fn mutable_binding_method_calls() {
  {
    let mut sample = Sample { key: 1 };
    sample.owned();
    // cannot use below calls because value has been moved
    // sample.owned();
    // sample.shared_ref();
    // sample.mutable_ref();
  }
  {
    let mut sample = Sample { key: 1 };
    sample.shared_ref();
    sample.shared_ref();
    sample.mutable_ref();
    sample.mutable_ref();
    sample.owned();
    // sample.owned();
  }
  {
    let mut sample = Sample { key: 1 };
    sample.shared_ref();
    sample.owned();
    // cannot call below because value has been moved
    // sample.mutable_ref();
  }
  {
    let mut sample = Sample { key: 1 };
    sample.mutable_ref();
    sample.mutable_ref();
    sample.shared_ref();
    sample.shared_ref();
    sample.owned();
  }
}
